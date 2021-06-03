import React, { useEffect } from "react";
import DraftContext from "../../context/DraftContext";
import IChampionStats from "../../models/IChampionStats";
import Ban from "./Ban";

interface IProps {
    teamStats: { [summonerName: string]: IChampionStats[] };
}

const Bans: React.FC<IProps> = ({ teamStats }) => {
    const [bans, setBans] = React.useState([]);

    useEffect(() => {
        if (!process.browser) {
            return;
        }

        import("league_of_clash/league_of_clash").then((loc) => {
            const bans = loc.get_bans(teamStats);
            setBans(bans);
        });
    }, []);

    return (
        <div>
            {bans.slice(0, 9).map((b, i) => (
                <Ban ban={b} key={Math.random()} />
            ))}
        </div>
    );
};

export default Bans;
