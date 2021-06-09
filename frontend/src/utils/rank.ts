export function getRankClass(tier: string) {
    tier = tier.toLowerCase();

    return {
        challenger: "text-rank-challenger",
        grandmaster: "text-rank-grandmaster",
        master: "text-rank-master",
        diamond: "text-rank-diamond",
        platinum: "text-rank-platinum",
        gold: "text-rank-gold",
        silver: "text-rank-silver",
        bronze: "text-rank-bronze",
        iron: "text-rank-iron",
    }[tier];
}
