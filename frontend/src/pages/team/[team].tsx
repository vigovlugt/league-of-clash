import { GetStaticProps } from "next";
import { useRouter } from "next/router";
import React, { useState } from "react";
import { getChampions } from "../../api/ddragon";
import Spinner from "../../components/svg/Spinner";
import Bans from "../../components/team/Bans";
import PlayerStats from "../../components/team/PlayerStats";
import { Phase } from "../../components/team/Phase";
import DraftContext from "../../context/DraftContext";
import IChampion from "../../models/IChampion";
import IChampionStats from "../../models/IChampionStats";

interface IProps {
    teamStats: { [summonerName: string]: IChampionStats[] };
    championData: { [key: number]: IChampion };
}

const PHASES = [
    "Scouting",
    "Ban phase 1",
    "Pick phase 1",
    "Ban phase 2",
    "Pick phase 2",
];

const TeamPage: React.FC<IProps> = ({ teamStats, championData }) => {
    const router = useRouter();

    if (router.isFallback) {
        return (
            <div className="h-screen flex justify-center items-center bg-dark">
                <Spinner />
            </div>
        );
    }

    const players = Object.keys(teamStats).sort();

    const [currentPhase, setPhase] = useState(0);

    return (
        <DraftContext.Provider value={{ championData }}>
            <div
                className="grid h-screen max-h-screen bg-dark"
                style={{
                    gridTemplateColumns: "auto 350px",
                    gridTemplateRows: "auto 65px",
                }}
            >
                <div className="bg-dark text-gray-50 p-4 overflow-y-auto">
                    <h1 className="font-header text-4xl text-primary">
                        <span className="uppercase">Clash team</span>
                    </h1>
                    <div
                        className="mt-4 grid gap-4"
                        style={{
                            gridTemplateColumns: "auto auto",
                        }}
                    >
                        {players.map((p) => (
                            <PlayerStats
                                summonerName={p}
                                championStats={teamStats[p]}
                                key={p}
                            ></PlayerStats>
                        ))}
                    </div>
                </div>

                <div
                    className="bg-primary w-full text-dark p-4 overflow-y-auto"
                    style={{
                        gridRow: "span 2 / auto",
                    }}
                >
                    <h2 className="font-header text-4xl uppercase">Bans</h2>
                    <Bans teamStats={teamStats} />
                </div>

                <div className="bg-dark w-full text-primary overflow-y-auto flex justify-between border-t border-gray-700 overflow-hidden">
                    <div className="flex items-center">
                        {PHASES.map((p, i) => (
                            <Phase
                                key={p}
                                name={p}
                                active={i === currentPhase}
                            />
                        ))}
                    </div>
                    <div className="flex justify-center items-center mr-4 ml-8">
                        <button
                            className="bg-primary text-dark py-2 px-3 text-xl font-header border-b-4 border-[#73571f] whitespace-nowrap focus:outline-none"
                            onClick={() => setPhase(currentPhase + 1)}
                        >
                            Next phase
                        </button>
                    </div>
                </div>
            </div>
        </DraftContext.Provider>
    );
};

export async function getStaticPaths() {
    return { paths: [], fallback: true };
}

async function getChampionStats(team: string) {
    const API_URL = process.env.API_URL;

    const players = team.split("+");

    const res = await fetch(
        API_URL + "api/championstats?team=" + players.join(",")
    );

    return await res.json();
}

export const getStaticProps: GetStaticProps = async (context) => {
    const team = context.params?.team?.toString();
    if (!team) {
        return {
            notFound: true,
        };
    }

    const [teamStats, championData] = await Promise.all([
        getChampionStats(team),
        getChampions(),
    ]);

    return {
        props: { teamStats, championData },
        revalidate: 60 * 60 * 12, // Revalidate in half a day.
    };
};

export default TeamPage;
function setState() {
    throw new Error("Function not implemented.");
}
