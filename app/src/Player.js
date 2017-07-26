export default class Player{
    contructor(name, uuid, isLeader, team){
      console.log("creating a player,", uuid);
        this.name = "";
        this.uuid = uuid;
        this.isLeader = isLeader;
        this.team = team;
    }
}