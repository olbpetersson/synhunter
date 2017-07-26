export default class Player{
    contructor(name, uuid, isLeader, team){
        console.log("creating a player,", uuid)
        this.name = "";
        this.isLeader = isLeader;
        this.uuid = uuid;
        this.team = team;
    }
}