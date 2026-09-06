// 活动池 HTTP API 测试脚本（绕开 PowerShell 中文编码坑）
// 用法（在本文件所在目录执行）：
//   node test-activity.mjs create                 创建演示活动（"正在下载" 15%）
//   node test-activity.mjs p <id> <progress>      更新进度，如: node test-activity.mjs p dl-demo 66
//   node test-activity.mjs delete <id>            删除活动
//   node test-activity.mjs list                   查看活动池
//   node test-activity.mjs clear                  清空活动池
const BASE = 'http://127.0.0.1:47300/api/activities';

async function main() {
    const [cmd, a, b] = process.argv.slice(2);
    const json = (o) => ({ method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(o) });

    try {
        if (cmd === 'create') {
            // POST 仅创建：同 id 已存在会 409，故先 DELETE（幂等）再创建，保证可重复演示
            await fetch(`${BASE}/dl-demo`, { method: 'DELETE' });
            const r = await fetch(BASE, json({
                id: 'dl-demo', title: '正在下载', subtitle: 'NetSpeed-Setup.exe',
                kind: '下载', progress: 15, priority: 10, ttl_ms: 60000
            }));
            console.log('创建结果:', JSON.stringify(await r.json()));
        } else if (cmd === 'p') {
            if (!a || !b) return console.log('用法: node test-activity.mjs p <id> <progress>');
            const r = await fetch(`${BASE}/${a}`, { method: 'PATCH', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ progress: Number(b) }) });
            console.log('更新结果:', JSON.stringify(await r.json()));
        } else if (cmd === 'delete') {
            if (!a) return console.log('用法: node test-activity.mjs delete <id>');
            const r = await fetch(`${BASE}/${a}`, { method: 'DELETE' });
            console.log('删除结果:', JSON.stringify(await r.json()));
        } else if (cmd === 'list') {
            const r = await fetch(BASE);
            const data = await r.json();
            console.log('活动池数量:', data.activities.length);
            data.activities.forEach((x) => console.log(`  [${x.id}] ${x.title} / ${x.subtitle} / ${x.kind} / ${x.progress ?? '-'}% / priority=${x.priority}`));
        } else if (cmd === 'clear') {
            const r = await fetch(BASE, { method: 'DELETE' });
            console.log('清空结果:', JSON.stringify(await r.json()));
        } else {
            console.log('未知命令。可用: create | p <id> <progress> | delete <id> | list | clear');
        }
    } catch (e) {
        console.error('请求失败（应用没在运行？）:', e.message);
    }
}

main();
