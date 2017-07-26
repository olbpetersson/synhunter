export default class Player {
    contructor(uuid, isLeader, team){
      console.log("creating a player,", uuid);
        this.uuid = uuid;
        this.isLeader = isLeader;
        this.team = team;
    }
}