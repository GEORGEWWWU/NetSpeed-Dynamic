(() => {
  const BRIDGE_URL = "ws://127.0.0.1:47391";
  const SEND_INTERVAL_MS = 250;
  const RECONNECT_INTERVAL_MS = 2000;

  let socket = null;
  let reconnectTimer = null;
  let sendTimer = null;
  let lastHeartbeatAt = 0;
  let nativeHooksReady = false;
  let getPlayingBinding = null;
  const nativeState = {
    song: "",
    artist: "",
    playing: false,
    position: 0,
    duration: 0,
    lastEvent: "none",
  };

  const normalizeArtists = (artists) => {
    if (typeof artists === "string") return artists.trim();
    if (!Array.isArray(artists)) return "";
    return artists
      .map((artist) => typeof artist === "string" ? artist : artist?.name)
      .filter(Boolean)
      .join("/");
  };

  const timeTextToMillis = (value) => {
    if (typeof value !== "string") return NaN;
    const parts = value.trim().split(":").map(Number);
    if (parts.length < 2 || parts.some((part) => !Number.isFinite(part))) return NaN;
    return parts.reduce((total, part) => total * 60 + part, 0) * 1000;
  };

  // BetterNCM 官方 InfinityLink 在网易云 3.x 使用的 DOM 信息源。
  const readDomPlayerState = () => {
    const song = document.querySelector(".cmd-space.title > span")?.innerText?.trim();
    const artist = document.querySelector(".cmd-space.title .author")?.innerText?.trim() ?? "";

    let currentText = document.querySelector(
      ".cmd-space.middle > div:nth-child(2) > p:nth-child(1)",
    )?.innerText;
    let durationText = document.querySelector(
      ".cmd-space.middle > div:nth-child(2) > p:nth-child(3)",
    )?.innerText;

    if (!currentText || !durationText) {
      const combinedTime = document.querySelector(
        ".cmd-space > div:nth-child(2) > p:nth-child(1)",
      )?.innerText;
      if (combinedTime?.includes("/")) {
        [currentText, durationText] = combinedTime.split("/").map((part) => part.trim());
      }
    }

    const position = timeTextToMillis(currentText);
    const duration = timeTextToMillis(durationText);
    if (!song || !Number.isFinite(position) || !Number.isFinite(duration) || duration <= 0) {
      return null;
    }

    return {
      type: "player-state",
      source: "netease",
      song,
      artist,
      playing: Boolean(document.querySelector(".cmd-icon-pause, .btnp-pause")),
      position,
      duration,
    };
  };

  const getDomDiagnostics = () => Array.from(document.querySelectorAll(
    '[class*="cmd-space"], [class*="DefaultBar"], [class*="PlayBar"], [class*="player"]',
  )).slice(0, 20).map((element) => ({
    tag: element.tagName,
    className: String(element.className).slice(0, 160),
    text: String(element.innerText ?? "").trim().replace(/\s+/g, " ").slice(0, 160),
  }));

  const refreshNativeSongInfo = () => {
    try {
      if (!getPlayingBinding) {
        const found = betterncm?.ncm?.findApiFunction?.("getPlaying");
        if (found) getPlayingBinding = [found[0], found[1]];
      }
      if (!getPlayingBinding) return;
      const playing = getPlayingBinding[0].call(getPlayingBinding[1]);
      const data = playing?.data;
      if (!data?.name) return;
      nativeState.song = String(data.name);
      nativeState.artist = normalizeArtists(data.artists);
      const duration = Number(data.duration ?? data.dt);
      if (Number.isFinite(duration) && duration > 0) {
        nativeState.duration = duration > 10000 ? duration : duration * 1000;
      }
    } catch (_) { }
  };

  // BetterNCM InfinityLink 使用的原生播放事件；界面 DOM 不可见时仍能拿到精确进度。
  const setupNativeHooks = () => {
    if (nativeHooksReady) return;
    nativeHooksReady = true;

    try {
      const rememberEvent = (name, args) => {
        nativeState.lastEvent = `${name}:${args.map((value) => {
          if (value && typeof value === "object") {
            try { return JSON.stringify(value).slice(0, 300); } catch (_) { return "[object]"; }
          }
          return String(value);
        }).join("|")}`;
      };

      channel.registerCall("audioplayer.onPlayProgress", (...args) => {
        rememberEvent("progress", args);
        const value = Number(args[1]);
        if (Number.isFinite(value)) nativeState.position = Math.max(0, Math.round(value * 1000));
      });
      channel.registerCall("audioplayer.onLoad", (...args) => {
        rememberEvent("load", args);
        const duration = Number(args[1]?.duration);
        if (Number.isFinite(duration)) nativeState.duration = Math.max(0, Math.round(duration * 1000));
        refreshNativeSongInfo();
      });
      channel.registerCall("audioplayer.onPlayState", (...args) => {
        rememberEvent("state", args);
        nativeState.playing = args[2] === 1;
      });

      legacyNativeCmder.appendRegisterCall("PlayProgress", "audioplayer", (_, progress) => {
        const value = Number(progress);
        if (Number.isFinite(value)) nativeState.position = Math.max(0, Math.round(value * 1000));
      });
      legacyNativeCmder.appendRegisterCall("Load", "audioplayer", (_, info) => {
        const duration = Number(info?.duration);
        if (Number.isFinite(duration)) nativeState.duration = Math.max(0, Math.round(duration * 1000));
        refreshNativeSongInfo();
      });
      legacyNativeCmder.appendRegisterCall("PlayState", "audioplayer", (_, __, state) => {
        nativeState.playing = state === 1;
      });
    } catch (_) { }

    try {
      const originalChannelCall = channel.call;
      channel.call = function (name, callback, args) {
        if (name === "player.setInfo" && args?.[0]) {
          const info = args[0];
          nativeState.song = String(info.songName ?? nativeState.song);
          nativeState.artist = String(info.artistName ?? nativeState.artist);
        }
        return originalChannelCall.apply(this, arguments);
      };
    } catch (_) { }

    refreshNativeSongInfo();
    setInterval(refreshNativeSongInfo, 1000);
  };

  const readNativePlayerState = () => {
    if (!nativeState.song || nativeState.duration <= 0) return null;
    return {
      type: "player-state",
      source: "netease",
      song: nativeState.song,
      artist: nativeState.artist,
      playing: nativeState.playing,
      position: nativeState.position,
      duration: nativeState.duration,
    };
  };

  const readPlayerState = () => {
    const api = window.__autotest__;
    if (!api) {
      const nativePlayerState = readNativePlayerState();
      if (nativePlayerState) {
        return { state: nativePlayerState, reason: "ok-native", apiKeys: [] };
      }
      const domState = readDomPlayerState();
      return domState
        ? { state: domState, reason: "ok-dom", apiKeys: [] }
        : { state: null, reason: "dom-player-data-missing", apiKeys: [] };
    }

    try {
      const info = api.playingInfo;
      const progress = api.playingProgress;
      if (!info?.name || !progress) {
        return {
          state: null,
          reason: "player-data-missing",
          apiKeys: Object.getOwnPropertyNames(api).slice(0, 30),
        };
      }

      const position = Number(progress.current);
      const duration = Number(progress.duration ?? info.duration);
      if (!Number.isFinite(position) || !Number.isFinite(duration)) {
        return {
          state: null,
          reason: `invalid-timeline:${String(progress.current)}/${String(progress.duration ?? info.duration)}`,
          apiKeys: Object.getOwnPropertyNames(api).slice(0, 30),
        };
      }

      return {
        state: {
          type: "player-state",
          source: "netease",
          song: String(info.name),
          artist: normalizeArtists(info.artists),
          playing: api.playingStatus === "Playing",
          position: Math.max(0, Math.round(position)),
          duration: Math.max(0, Math.round(duration)),
        },
        reason: "ok",
        apiKeys: [],
      };
    } catch (error) {
      return {
        state: null,
        reason: `exception:${error?.message ?? String(error)}`,
        apiKeys: Object.getOwnPropertyNames(api).slice(0, 30),
      };
    }
  };

  const stopSending = () => {
    if (sendTimer !== null) {
      clearInterval(sendTimer);
      sendTimer = null;
    }
  };

  const scheduleReconnect = () => {
    if (reconnectTimer !== null) return;
    reconnectTimer = setTimeout(() => {
      reconnectTimer = null;
      connect();
    }, RECONNECT_INTERVAL_MS);
  };

  const connect = () => {
    if (socket && (socket.readyState === WebSocket.OPEN || socket.readyState === WebSocket.CONNECTING)) {
      return;
    }

    try {
      socket = new WebSocket(BRIDGE_URL);
      socket.addEventListener("open", () => {
        stopSending();
        const sendState = () => {
          if (socket?.readyState !== WebSocket.OPEN) return;
          const result = readPlayerState();
          if (result.state) {
            socket.send(JSON.stringify(result.state));
          } else if (Date.now() - lastHeartbeatAt >= 2000) {
            lastHeartbeatAt = Date.now();
            socket.send(JSON.stringify({
              type: "bridge-heartbeat",
              source: "netease",
              reason: result.reason,
              apiKeys: result.apiKeys,
              dom: getDomDiagnostics(),
              native: {
                song: nativeState.song,
                position: nativeState.position,
                duration: nativeState.duration,
                playing: nativeState.playing,
                lastEvent: nativeState.lastEvent,
              },
            }));
          }
        };
        sendState();
        sendTimer = setInterval(sendState, SEND_INTERVAL_MS);
      });
      socket.addEventListener("close", () => {
        stopSending();
        scheduleReconnect();
      });
      socket.addEventListener("error", () => socket?.close());
    } catch (error) {
      console.warn("[NetSpeed Dynamic Sync] 连接失败", error);
      scheduleReconnect();
    }
  };

  plugin.onLoad(() => {
    setupNativeHooks();
    connect();
  });
})();
