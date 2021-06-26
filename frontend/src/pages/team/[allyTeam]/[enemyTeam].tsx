import { GetStaticProps } from "next";
import { useRouter } from "next/router";
import React, { useEffect, useState } from "react";
import { getChampions } from "../../../api/ddragon";
import Spinner from "../../../components/svg/Spinner";
import PlayerStats from "../../../components/team/PlayerStats";
// import { Phase } from "../../../components/team/Phase";
import IChampion from "../../../models/IChampion";
import IPlayerStats from "../../../models/IPlayerStats";
import useStore from "../../../store/DraftStore";
import PickBanPhasePanel from "../../../components/team/PickBanPhasePanel";
import Team from "../../../models/Team";
import Bans from "../../../components/ban/Bans";
import { DndProvider } from "react-dnd";
import { HTML5Backend } from "react-dnd-html5-backend";
import WebSocketManager from "../../../modules/websocket/WebSocketManager";
import { Phase, PHASES } from "../../../models/Phase";
import PhaseComponent from "../../../components/team/Phase";

interface IProps {
    allyPlayerStats: { [player: string]: IPlayerStats };
    enemyPlayerStats: { [player: string]: IPlayerStats };
    championData: { [key: number]: IChampion };
}

const TeamPage: React.FC<IProps> = ({
    allyPlayerStats,
    enemyPlayerStats,
    championData,
}) => {
    const router = useRouter();

    const setChampionData = useStore((store) => store.setChampionData);
    const setPlayerStats = useStore((store) => store.setPlayerStats);
    setChampionData(championData);
    setPlayerStats(allyPlayerStats, enemyPlayerStats);

    const setLeagueOfClash = useStore((store) => store.setLeagueOfClash);
    const setWebSocketManager = useStore((store) => store.setWebSocketManager);
    useEffect(() => {
        if (process.browser && enemyPlayerStats) {
            import("league-of-clash").then((loc) => {
                setLeagueOfClash(loc, enemyPlayerStats);
            });

            const allyTeam = Object.keys(allyPlayerStats).sort().join("+");
            const enemyTeam = Object.keys(enemyPlayerStats).sort().join("+");

            setWebSocketManager(new WebSocketManager(allyTeam, enemyTeam));
        }
    }, [enemyPlayerStats, allyPlayerStats]);

    const phase = useStore((store) => store.phase);
    const nextPhase = useStore((store) => store.nextPhase);

    const [activeTeam, setActiveTeam] = useState(Team.Enemy);

    if (router.isFallback) {
        return (
            <div className="h-screen flex justify-center items-center bg-dark">
                <Spinner />
            </div>
        );
    }

    return (
        <DndProvider backend={HTML5Backend}>
            <div
                className="grid h-screen max-h-screen bg-dark"
                style={{
                    gridTemplateColumns: "auto 350px",
                    gridTemplateRows: "auto 65px",
                }}
            >
                <div
                    className="bg-dark text-gray-50 overflow-y-hidden grid"
                    style={{
                        gridTemplateColumns:
                            phase !== Phase.SCOUT_PHASE && phase !== Phase.GAME
                                ? "1fr 2fr 1fr"
                                : "auto",
                    }}
                >
                    {phase !== Phase.SCOUT_PHASE && phase !== Phase.GAME && (
                        <PickBanPhasePanel team={Team.Ally} />
                    )}
                    <div className="px-4 py-3 h-full overflow-y-auto">
                        {/* {phase != SCOUT_PHASE && phase != GAME && (
                            <PickBanSummary />
                        )} */}

                        <div className="flex space-x-2 items-end mb-2">
                            <h2
                                className={`font-header uppercase cursor-pointer ${
                                    activeTeam === Team.Enemy
                                        ? "text-4xl text-primary"
                                        : "text-2xl text-gray-400"
                                }`}
                                onClick={() => setActiveTeam(Team.Enemy)}
                            >
                                Enemy Team
                            </h2>
                            <h2
                                className={`font-header uppercase cursor-pointer ${
                                    activeTeam === Team.Ally
                                        ? "text-4xl text-primary"
                                        : "text-2xl text-gray-400"
                                }`}
                                onClick={() => setActiveTeam(Team.Ally)}
                            >
                                Ally Team
                            </h2>
                        </div>

                        <div
                            className="grid gap-4"
                            style={{
                                gridTemplateColumns:
                                    phase == Phase.SCOUT_PHASE ||
                                    phase == Phase.GAME
                                        ? "auto auto"
                                        : "auto",
                            }}
                        >
                            {Object.values(
                                activeTeam == Team.Ally
                                    ? allyPlayerStats
                                    : enemyPlayerStats
                            )
                                .sort((a, b) =>
                                    a.summoner_name.localeCompare(
                                        b.summoner_name
                                    )
                                )
                                .map((p) => (
                                    <PlayerStats
                                        playerStats={p}
                                        key={p.summoner_name}
                                    ></PlayerStats>
                                ))}
                        </div>
                    </div>
                    {phase !== Phase.SCOUT_PHASE && phase !== Phase.GAME && (
                        <PickBanPhasePanel team={Team.Enemy} />
                    )}
                </div>

                <div
                    className="bg-primary w-full text-dark p-4 overflow-y-auto"
                    style={{
                        gridRow: "span 2 / auto",
                    }}
                >
                    <h2 className="font-header text-3xl uppercase">
                        Recommended Bans
                    </h2>
                    <Bans playerStats={enemyPlayerStats} />
                </div>

                <div className="bg-dark w-full text-primary overflow-y-auto flex justify-between border-t border-gray-700 overflow-hidden">
                    <div className="flex items-center">
                        {PHASES.map((p) => (
                            <PhaseComponent
                                key={p.type}
                                name={p.name}
                                active={p.type === phase}
                            />
                        ))}
                    </div>
                    <div className="flex justify-center items-center mr-4 ml-8">
                        {phase != PHASES[PHASES.length - 1].type && (
                            <button
                                className="bg-primary text-dark py-2 px-3 text-xl font-header border-b-4 border-primary-dark whitespace-nowrap uppercase active:border-b-0 active:mt-1 focus:outline-none"
                                onClick={() => nextPhase()}
                            >
                                Next phase
                            </button>
                        )}
                    </div>
                </div>
            </div>
        </DndProvider>
    );
};

export async function getStaticPaths() {
    return { paths: [], fallback: true };
}

async function getPlayerStats(team: string) {
    const API_URL = process.env.NEXT_PUBLIC_API_URL;

    const res = await fetch(
        API_URL +
            "api/team/euw1/" +
            team
                .split("+")
                .map((p) => encodeURIComponent(p))
                .join("+")
    );

    return await res.json();
}

export const getStaticProps: GetStaticProps = async (context) => {
    const allyTeam = context.params?.allyTeam?.toString();
    const enemyTeam = context.params?.enemyTeam?.toString();
    if (!allyTeam || !enemyTeam) {
        return {
            notFound: true,
        };
    }

    const [allyPlayerStats, enemyPlayerStats, championData] = await Promise.all(
        [getPlayerStats(allyTeam), getPlayerStats(enemyTeam), getChampions()]
    );

    return {
        props: { allyPlayerStats, enemyPlayerStats, championData },
        revalidate: 60 * 60 * 12, // Revalidate in half a day.
    };
};

export default TeamPage;
