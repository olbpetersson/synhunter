import 'vue-material-css';

import Vue from 'vue';
import VueMaterial from 'vue-material';

import Board from 'Board.vue';
import GameInput from 'GameInput.vue';

Vue.use(VueMaterial);

const app = new Vue({
  el: '#app',
  components: {Board, GameInput}
});