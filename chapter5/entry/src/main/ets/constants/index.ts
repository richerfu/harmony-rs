import relationalStore from '@ohos.data.relationalStore';

export const CREATE_SQL = `CREATE TABLE IF NOT EXISTS todos (
    id TEXT PRIMARY KEY,
    content TEXT NOT NULL,
    status INTEGER NOT NULL,
    is_delete INTEGER NOT NULL
)`;

export const STORE_CONFIG =  {
  name: 'todomvc.db', // 数据库文件名
  securityLevel: relationalStore.SecurityLevel.S1 // 数据库安全级别
};