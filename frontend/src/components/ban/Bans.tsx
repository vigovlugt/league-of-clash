import React, { useEffect } from "react";
import IPlayerStats from "../../models/IPlayerStats";
import Ban from "./Ban";

interface IProps {
    playerStats: IPlayerStats[];
}

const Bans: React.FC<IProps> = ({ playerStats }) => {
    const [bans, setBans] = React.useState([]);

    useEffect(() => {
        if (!process.browser) {
            return;
        }

        // @ts-ignore
        import("league-of-clash").then((loc) => {
            const bans = loc.get_bans(playerStats);
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
