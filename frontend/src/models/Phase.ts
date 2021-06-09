export const SCOUT_PHASE = "SCOUT_PHASE";
export const BAN_PHASE_1 = "BAN_PHASE_1";
export const PICK_PHASE_1 = "PICK_PHASE_1";
export const BAN_PHASE_2 = "BAN_PHASE_2";
export const PICK_PHASE_2 = "PICK_PHASE_2";
export const GAME = "GAME";

export interface IPhase {
    id: string;
    name: string;
}

export const PHASES: IPhase[] = [
    { name: "Scouting", id: SCOUT_PHASE },
    { name: "Ban phase 1", id: BAN_PHASE_1 },
    { name: "Pick phase 1", id: PICK_PHASE_1 },
    { name: "Ban phase 2", id: BAN_PHASE_2 },
    { name: "Pick phase 2", id: PICK_PHASE_2 },
    { name: "Game", id: GAME },
];
