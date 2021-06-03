import { GetStaticProps } from "next";
import React from "react";
import { getChampions } from "../../api/ddragon";
import Bans from "../../components/team/Bans";
import PlayerStats from "../../components/team/PlayerStats";
import DraftContext from "../../context/DraftContext";
import IChampion from "../../models/IChampion";
import IChampionStats from "../../models/IChampionStats";

interface IProps {
    teamStats: { [summonerName: string]: IChampionStats[] };
    championData: { [key: number]: IChampion };
}

const TeamPage: React.FC<IProps> = ({ teamStats, championData }) => {
    const players = Object.keys(teamStats).sort();

    return (
        <DraftContext.Provider value={{ championData }}>
            <div
                className="grid h-screen max-h-screen bg-dark"
                style={{
                    gridTemplateColumns: "auto 400px",
                    gridTemplateRows: "auto",
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

                <div className="bg-primary w-full text-dark p-4 overflow-y-auto">
                    <h2 className="font-header text-4xl uppercase">Bans</h2>
                    <Bans teamStats={teamStats} />
                </div>
            </div>
        </DraftContext.Provider>
    );
};

export async function getStaticPaths() {
    return { paths: [], fallback: "blocking" }; // TODO: make not blocking
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
