export enum MessageType {
    SYNC_STATE = "SYNC_STATE",
}

export default class Message {
    constructor(public type: MessageType, public data: any) {}
}
