import { useRouter } from "next/router";
import Head from "next/head";
import { FormEvent, useState } from "react";
import ClashLogo from "../components/svg/ClashLogo";
import PersonIcon from "../components/svg/PersonIcon";
import TeamHeader from "../components/svg/TeamHeader";
import TeamIcon from "../components/svg/TeamIcon";
import toast from "react-hot-toast";

export default function IndexPage() {
    const router = useRouter();

    const [allyTeam, setAllyTeam] = useState("");
    const [enemyTeam, setEnemyTeam] = useState("");
    const [player, setPlayer] = useState("");

    const getPlayers = (team: string) =>
        [
            ...new Set(
                team
                    .split(",")
                    .map((p) => p.trim())
                    .filter((p) => p)
            ),
        ].sort((a, b) => a.localeCompare(b));

    const createToast = (str: string) =>
        toast(str, {
            style: {
                background: "#202020",
                color: "#fff",
            },
            position: "top-right",
        });

    const searchTeam = (e: FormEvent) => {
        e.preventDefault();
        const allyPlayers = getPlayers(allyTeam);
        const enemyPlayers = getPlayers(enemyTeam);

        if (allyPlayers.length != 5) {
            return createToast("Ally team doesn't have 5 players");
        }

        if (enemyPlayers.length != 5) {
            return createToast("Enemy team doesn't have 5 players");
        }

        router.push(`/team/${allyPlayers.join("+")}/${enemyPlayers.join("+")}`);
    };

    const searchPlayer = (e: FormEvent) => {
        e.preventDefault();
    };

    return (
        <div className="min-h-screen flex flex-col lg:flex-row">
            <div className="relative w-full h-screen">
                <div className="bg-dark font-header flex justify-center items-center z-10 absolute inset-0 overflow-hidden">
                    <ClashLogo className="w-[800px] absolute text-white opacity-5 left-1/2 top-1/2 transform -translate-x-1/2 -translate-y-1/2" />

                    <h1 className="text-8xl font-bold tracking-wider text-primary z-20">
                        LEAGUE OF{" "}
                        <span className="block text-[10.25rem]">CLASH</span>
                    </h1>
                </div>
                <svg
                    className="hidden lg:block absolute right-0 inset-y-0 h-full w-48 text-dark transform translate-x-1/2"
                    fill="currentColor"
                    viewBox="0 0 100 100"
                    preserveAspectRatio="none"
                    aria-hidden="true"
                >
                    <polygon points="50,0 100,0 50,100 0,100"></polygon>
                </svg>
            </div>

            <div className="bg-primary w-full flex justify-center items-center text-dark h-screen">
                <div className="flex flex-col items-center">
                    <h2 className="text-5xl font-header uppercase mb-4">
                        Clash
                    </h2>
                    <TeamHeader className="w-96 text-dark">
                        <TeamIcon className="w-6" />
                    </TeamHeader>

                    <form className="mt-2 w-full" onSubmit={searchTeam}>
                        <input
                            className="bg-dark rounded-md p-3 text-white focus:outline-none w-full placeholder-white placeholder-opacity-30"
                            placeholder="Ally team"
                            onChange={(e) => setAllyTeam(e.target.value)}
                            value={allyTeam}
                            name="clash-team"
                        />
                        <input
                            className="bg-dark rounded-md p-3 text-white focus:outline-none w-full placeholder-white placeholder-opacity-30 mt-4"
                            placeholder="Enemy team"
                            onChange={(e) => setEnemyTeam(e.target.value)}
                            value={enemyTeam}
                            name="clash-team"
                        />
                        <button
                            type="submit"
                            className="font-header text-dark text-lg text-center px-3 py-2 mt-4 rounded-md border-2 border-dark w-full"
                        >
                            Go
                        </button>
                    </form>
                    {/* TODO:REGION */}

                    <h2 className="text-5xl font-header uppercase mb-4 mt-32">
                        Search Player
                    </h2>
                    <TeamHeader className="w-96 text-dark">
                        <PersonIcon className="w-6" />
                    </TeamHeader>

                    <form className="mt-2 w-full" onSubmit={searchPlayer}>
                        <input
                            className="bg-dark rounded-md p-3 text-white focus:outline-none w-full placeholder-white placeholder-opacity-30"
                            placeholder="HULKSMASH1337"
                            onChange={(e) => setPlayer(e.target.value)}
                            value={player}
                        />
                    </form>
                </div>
            </div>
        </div>
    );
}
