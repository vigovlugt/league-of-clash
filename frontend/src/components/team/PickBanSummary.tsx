import React from "react";
import Action from "../../models/Action";
import IChampion from "../../models/IChampion";
import Team from "../../models/Team";
import useStore from "../../store/DraftStore";

const DDRAGON_URL = process.env.NEXT_PUBLIC_DDRAGON_URL;

const PickBanSummary = () => {
    return (
        <div className="flex justify-between space-x-4 mb-4">
            <PickBanSummaryTeam team={Team.Ally} />

            <PickBanSummaryTeam team={Team.Enemy} />
        </div>
    );
};

interface ITeamProps {
    team: Team;
}

const PickBanSummaryTeam: React.FC<ITeamProps> = ({ team }) => {
    const isAlly = team === Team.Ally;
    const picks = useStore(
        (store) => store[isAlly ? "allyPicks" : "enemyPicks"]
    );
    const bans = useStore((store) => store[isAlly ? "allyBans" : "enemyBans"]);

    return (
        <div
            className={`flex flex-col space-y-2 ${
                isAlly ? "items-start" : "items-end"
            }`}
        >
            <div className="flex space-x-2 w-min">
                {picks.map((p, i) => (
                    <PickBanSummaryChampion
                        key={p + "" + i}
                        championId={p}
                        type={Action.Pick}
                    />
                ))}
            </div>
            <div className="flex space-x-2 w-min">
                {bans.map((p, i) => (
                    <PickBanSummaryChampion
                        key={p + "" + i}
                        championId={p}
                        type={Action.Ban}
                    />
                ))}
            </div>
        </div>
    );
};

interface IChampionProps {
    championId: string | null;
    type: Action;
}

const PickBanSummaryChampion: React.FC<IChampionProps> = ({
    championId,
    type,
}) => {
    const isPick = type === Action.Pick;

    const championData = useStore((store) => store.championData);
    const champion = championId != null ? championData[championId] : null;

    const sizeClass = isPick
        ? "w-12 h-12 min-w-12 min-h-12"
        : "w-8 h-8 min-w-8 min-h-8";

    return (
        <div className={`relative overflow-hidden ${sizeClass}`}>
            {champion != null ? (
                <img
                    src={`${DDRAGON_URL}cdn/11.11.1/img/champion/${champion.id}.png`}
                    className={`absolute inset-0 ${sizeClass} ${
                        isPick ? "" : "filter grayscale"
                    }`}
                    style={{ transform: "scale(1.1,1.1)" }}
                />
            ) : (
                <div
                    className={`absolute inset-0 bg-light-dark ${sizeClass}`}
                />
            )}
        </div>
    );
};

export default PickBanSummary;
