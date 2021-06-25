import React, { useEffect, useState } from "react";
import Action from "../../models/Action";
import IChampionRolePlayerPrediction from "../../models/IChampionRolePlayerPrediction";
import Team from "../../models/Team";
import useStore from "../../store/DraftStore";
import BanPanel from "../ban/BanPanel";
import ChampionPanel from "../champion/ChampionPanel";
import { ChampionSelect } from "../champion/ChampionSelect";
import ChampionSelectPanel from "../champion/ChampionSelectPanel";

interface IProps {
    team: Team;
}

const PickBanPhasePanel: React.FC<IProps> = ({ team }) => {
    const isAllyTeam = team === Team.Ally;

    const leagueOfClash = useStore((store) => store.leagueOfClash);
    const rolePlayerPredictor = useStore(
        (store) => store.roleChampionPlayerPredictor
    );
    const playerStats = useStore(
        (store) => store[isAllyTeam ? "allyPlayerStats" : "enemyPlayerStats"]
    );

    const bans = useStore(
        (store) => store[isAllyTeam ? "allyBans" : "enemyBans"]
    );
    const setBan = useStore(
        (store) => store[isAllyTeam ? "setAllyBan" : "setEnemyBan"]
    );

    const picks = useStore(
        (store) => store[isAllyTeam ? "allyPicks" : "enemyPicks"]
    );
    const setPick = useStore(
        (store) => store[isAllyTeam ? "setAllyPick" : "setEnemyPick"]
    );

    const [predictions, setPredictions] = useState<{
        [championId: number]: IChampionRolePlayerPrediction[];
    }>({});

    useEffect(() => {
        if (
            isAllyTeam ||
            !leagueOfClash ||
            !rolePlayerPredictor ||
            !picks.filter((p) => p).length ||
            !playerStats
        ) {
            return;
        }

        const predictions = leagueOfClash.get_predictions(
            rolePlayerPredictor,
            {
                players: Object.keys(playerStats),
                region: "euw1",
            },
            {
                champion_ids: picks.filter((p) => p != null).map((p) => +p!),
            }
        );

        setPredictions(predictions);
    }, [leagueOfClash, rolePlayerPredictor, picks, team, playerStats]);

    const [selectedIndex, setSelectedIndex] = useState<number | null>(null);
    const [selectedAction, setSelectedAction] = useState<Action>(Action.Pick);

    const onClickBanPanel = (i: number) => {
        setSelectedIndex(i);
        setSelectedAction(Action.Ban);
    };

    const onClickChampionPanel = (i: number) => setSelectedIndex(i);
    const onCloseChampionSelect = () => setSelectedIndex(null);

    const onSelectChampion = (id: number) => {
        if (selectedAction === Action.Pick) {
            setPick(selectedIndex as number, id);
        } else {
            setBan(selectedIndex as number, id);
        }
        setSelectedIndex(null);
    };

    if (selectedIndex != null) {
        return (
            <ChampionSelectPanel
                onSelect={onSelectChampion}
                onClose={onCloseChampionSelect}
            ></ChampionSelectPanel>
        );
    }

    return (
        <div className="h-full overflow-auto flex flex-col">
            <h2
                className={`px-4 py-3 font-header text-4xl text-primary border-b-2 border-gray-700 uppercase ${
                    isAllyTeam ? "border-r-2" : "border-l-2"
                }`}
            >
                {isAllyTeam ? "Ally" : "Enemy"} Team
            </h2>
            <div className="flex flex-col h-full">
                {[...new Array(5)].map((_, i) => (
                    <ChampionPanel
                        key={i}
                        type={Action.Pick}
                        championId={picks[i]}
                        predictions={
                            picks[i] != null ? predictions[picks[i]!] : []
                        }
                        team={team}
                        index={i}
                        onClick={() => onClickChampionPanel(i)}
                    ></ChampionPanel>
                ))}
                <div
                    className={`flex px-2 ${
                        team == Team.Ally ? "border-r-2" : "border-l-2"
                    } border-gray-700`}
                >
                    {[...new Array(5)].map((_, i) => (
                        <BanPanel
                            key={i}
                            championId={bans[i]}
                            team={team}
                            index={i}
                            onClick={() => onClickBanPanel(i)}
                        ></BanPanel>
                    ))}
                </div>
            </div>
        </div>
    );
};

export default PickBanPhasePanel;
