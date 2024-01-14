function generateUniqueId() {
  return (Math.random() + 1).toString(36).substr(2);
}

@Observed
export class Todo {
  // 当前记录的 id
  public id: string;
  // 当前记录的内容
  public content: string;
  // 当前记录状态 0: 未完成 1: 已完成
  public status: number;
  // 是否删除  0: 未删除 1: 已删除
  public isDelete: number;

  constructor(content: string,id?: string,status?: number,isDelete?: number) {
    this.id = id || generateUniqueId();
    this.content = content;
    this.status = status || 0;
    this.isDelete = isDelete || 0;
  }
}