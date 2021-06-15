import create from "zustand";
import Action from "../models/Action";
import IChampion from "../models/IChampion";
import IPlayerStats from "../models/IPlayerStats";
import { PHASES, SCOUT_PHASE } from "../models/Phase";
import Team from "../models/Team";

type DraftState = {
    leagueOfClash: any;
    roleChampionPlayerPredictor: any;

    setLeagueOfClash: (leagueOfClash: any, playerStats: any) => void;

    allyPlayerStats: { [player: string]: IPlayerStats };
    enemyPlayerStats: { [player: string]: IPlayerStats };
    setPlayerStats: (
        allyPlayerStats: { [player: string]: IPlayerStats },
        enemyPlayerStats: { [player: string]: IPlayerStats }
    ) => void;

    championData: { [key: string]: IChampion };
    setChampionData: (championData: { [key: string]: IChampion }) => void;

    phase: string;
    setPhase: (phase: string) => void;
    nextPhase: () => void;

    allyBans: (number | null)[];
    setAllyBan: (i: number, id: number) => void;
    enemyBans: (number | null)[];
    setEnemyBan: (i: number, id: number) => void;

    allyPicks: (number | null)[];
    setAllyPick: (i: number, id: number) => void;
    enemyPicks: (number | null)[];
    setEnemyPick: (i: number, id: number) => void;

    getPickBannedChampions: () => number[];
    setPickBan: (
        type: Action,
        team: Team,
        index: number,
        championId: number
    ) => void;
};

const nextPhase = (phase: string) =>
    PHASES[PHASES.findIndex((p) => p.id === phase) + 1].id;

const useStore = create<DraftState>((set, get) => ({
    leagueOfClash: null,
    roleChampionPlayerPredictor: null,
    setLeagueOfClash: async (leagueOfClash: any, playerStats: any) => {
        const res = await fetch("/dataset.json");
        const text = await res.text();

        const predictor = leagueOfClash.create_predictor(playerStats, text);

        set(() => ({ leagueOfClash, roleChampionPlayerPredictor: predictor }));
    },

    allyPlayerStats: {},
    enemyPlayerStats: {},
    setPlayerStats: (
        allyPlayerStats: { [player: string]: IPlayerStats },
        enemyPlayerStats: { [player: string]: IPlayerStats }
    ) => set(() => ({ allyPlayerStats, enemyPlayerStats })),

    championData: {},
    setChampionData: (championData: { [key: string]: IChampion }) =>
        set(() => ({ championData })),

    phase: SCOUT_PHASE,
    setPhase: (phase: string) => set(() => ({ phase })),
    nextPhase: () => set((store) => ({ phase: nextPhase(store.phase) })),

    allyBans: [null, null, null, null, null],
    setAllyBan: (i: number, id: number) =>
        set((store) => {
            const allyBans = [...store.allyBans];
            allyBans[i] = id;

            return { allyBans };
        }),

    enemyBans: [null, null, null, null, null],
    setEnemyBan: (i: number, id: number) =>
        set((store) => {
            const enemyBans = [...store.enemyBans];
            enemyBans[i] = id;

            return { enemyBans };
        }),

    allyPicks: [null, null, null, null, null],
    setAllyPick: (i: number, id: number) =>
        set((store) => {
            const allyPicks = [...store.allyPicks];
            allyPicks[i] = id;

            return { allyPicks };
        }),

    enemyPicks: [null, null, null, null, null],
    setEnemyPick: (i: number, id: number) =>
        set((store) => {
            const enemyPicks = [...store.enemyPicks];
            enemyPicks[i] = id;

            return { enemyPicks };
        }),

    getPickBannedChampions: () => {
        const { allyBans, enemyBans, allyPicks, enemyPicks } = get();

        return [...allyBans, ...enemyBans, ...allyPicks, ...enemyPicks].filter(
            (c) => c != null
        ) as number[];
    },

    setPickBan: (
        type: Action,
        team: Team,
        index: number,
        championId: number
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
