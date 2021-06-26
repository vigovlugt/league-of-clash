import { UseStore } from "zustand";
import shallow from "zustand/shallow";
import { Phase } from "../../models/Phase";
import useStore, { DraftState } from "../../store/DraftStore";
import Message, { MessageType } from "./Message";

const API_URL = process.env.NEXT_PUBLIC_API_URL;

type PickBans = (number | null)[];

interface MessageEvent extends Event {
    readonly data: string;
}

export default class WebSocketManager {
    static instance: WebSocketManager;

    private connection: WebSocket;
    private store: UseStore<DraftState>;

    private flushedDraftState: boolean = true;

    constructor(allyTeam: string, enemyTeam: string) {
        this.connection = new WebSocket(
            API_URL!.replace("http", "ws") +
                `ws/room/euw1/${allyTeam}/${enemyTeam}`
        );
        this.connection.onopen = (e: Event) => this.onConnect(e);
        this.connection.onmessage = (e: MessageEvent) => this.onMessage(e);

        this.store = useStore;
        this.store.subscribe(
            (stateSlice) => this.onStoreChange(stateSlice),
            (state: DraftState) => ({
                allyBans: state.allyBans,
                allyPicks: state.allyPicks,
                enemyBans: state.enemyBans,
                enemyPicks: state.enemyPicks,
                phase: state.phase,
            }),
            shallow
        );

        if (WebSocketManager.instance) {
            WebSocketManager.instance.disconnect();
        }
        WebSocketManager.instance = this;
    }

    disconnect() {
        this.connection.close();
    }

    onConnect(e: Event) {
        console.log("WS CONNECTED");
    }

    onMessage(e: MessageEvent) {
        const message = JSON.parse(e.data) as Message;

        switch (message.type) {
            case MessageType.SYNC_STATE:
                this.onSyncState(message.data);
                break;
        }
    }

    onStoreChange(draftState: any) {
        if (!this.flushedDraftState) {
            this.flushedDraftState = true;
            return;
        }

        this.sendMessage(new Message(MessageType.SYNC_STATE, draftState));
    }

    onSyncState(data: any) {
        this.flushedDraftState = false;
        this.store.getState().setDraftState(data);
    }

    sendMessage(message: Message) {
        this.connection.send(JSON.stringify(message));
    }
}
