import React, { useEffect } from "react";
import IPlayerStats from "../../models/IPlayerStats";
import useStore from "../../store/DraftStore";
import Ban from "./Ban";

interface IProps {
    playerStats: { [player: string]: IPlayerStats };
}

const Bans: React.FC<IProps> = ({ playerStats }) => {
    const [bans, setBans] = React.useState([]);

    const leagueOfClash = useStore((store) => store.leagueOfClash);

    useEffect(() => {
        if (!leagueOfClash) {
            return;
        }

        const bans = leagueOfClash.get_bans(playerStats);
        setBans(bans);
    }, [leagueOfClash]);

    return (
        <div>
            {bans.slice(0, 9).map((b, i) => (
                <Ban ban={b} key={Math.random()} />
            ))}
        </div>
    );
};

export default Bans;
