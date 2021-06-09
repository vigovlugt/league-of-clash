import create from "zustand";
import Action from "../models/Action";
import IChampion from "../models/IChampion";
import { PHASES, SCOUT_PHASE } from "../models/Phase";
import Team from "../models/Team";

type DraftState = {
    championData: { [key: string]: IChampion };
    phase: string;

    setChampionData: (championData: { [key: string]: IChampion }) => void;
    setPhase: (phase: string) => void;
    nextPhase: () => void;

    allyBans: (string | null)[];
    setAllyBan: (i: number, id: string) => void;
    enemyBans: (string | null)[];
    setEnemyBan: (i: number, id: string) => void;

    allyPicks: (string | null)[];
    setAllyPick: (i: number, id: string) => void;
    enemyPicks: (string | null)[];
    setEnemyPick: (i: number, id: string) => void;

    getPickBannedChampions: () => string[];
    setPickBan: (
        type: Action,
        team: Team,
        index: number,
        championId: string
    ) => void;
};

const nextPhase = (phase: string) =>
    PHASES[PHASES.findIndex((p) => p.id === phase) + 1].id;

const useStore = create<DraftState>((set, get) => ({
    championData: {},
    setChampionData: (championData: { [key: string]: IChampion }) =>
        set(() => ({ championData })),

    phase: SCOUT_PHASE,
    setPhase: (phase: string) => set(() => ({ phase })),
    nextPhase: () => set((store) => ({ phase: nextPhase(store.phase) })),

    allyBans: [null, null, null, null, null],
    setAllyBan: (i: number, id: string) =>
        set((store) => {
            const allyBans = [...store.allyBans];
            allyBans[i] = id;

            return { allyBans };
        }),

    enemyBans: [null, null, null, null, null],
    setEnemyBan: (i: number, id: string) =>
        set((store) => {
            const enemyBans = [...store.enemyBans];
            enemyBans[i] = id;

            return { enemyBans };
        }),

    allyPicks: [null, null, null, null, null],
    setAllyPick: (i: number, id: string) =>
        set((store) => {
            const allyPicks = [...store.allyPicks];
            allyPicks[i] = id;

            return { allyPicks };
        }),

    enemyPicks: [null, null, null, null, null],
    setEnemyPick: (i: number, id: string) =>
        set((store) => {
            const enemyPicks = [...store.enemyPicks];
            enemyPicks[i] = id;

            return { enemyPicks };
        }),

    getPickBannedChampions: () => {
        const { allyBans, enemyBans, allyPicks, enemyPicks } = get();

        return [...allyBans, ...enemyBans, ...allyPicks, ...enemyPicks].filter(
            (c) => c != null
        ) as string[];
    },

    setPickBan: (
        type: Action,
        team: Team,
        index: number,
        championId: string
    ) => {
        const store = get();
        const set = {
            [Action.Ban]: {
                [Team.Ally]: store.setAllyBan,
                [Team.Enemy]: store.setEnemyBan,
            },
            [Action.Pick]: {
                [Team.Ally]: store.setAllyPick,
                [Team.Enemy]: store.setEnemyPick,
            },
        }[type][team];

        set(index, championId);
    },
}));

export default useStore;
