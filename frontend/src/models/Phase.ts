export enum Phase {
    SCOUT_PHASE = "SCOUT_PHASE",
    BAN_PHASE_1 = "BAN_PHASE_1",
    PICK_PHASE_1 = "PICK_PHASE_1",
    BAN_PHASE_2 = "BAN_PHASE_2",
    PICK_PHASE_2 = "PICK_PHASE_2",
    GAME = "GAME",
}

export interface IPhase {
    type: Phase;
    name: string;
}

export const PHASES: IPhase[] = [
    { name: "Scouting", type: Phase.SCOUT_PHASE },
    { name: "Ban phase 1", type: Phase.BAN_PHASE_1 },
    { name: "Pick phase 1", type: Phase.PICK_PHASE_1 },
    { name: "Ban phase 2", type: Phase.BAN_PHASE_2 },
    { name: "Pick phase 2", type: Phase.PICK_PHASE_2 },
    { name: "Game", type: Phase.GAME },
];
