import 'vue-material-css';

import Vue from 'vue';
import VueMaterial from 'vue-material';

import Board from 'Board.vue';

Vue.use(VueMaterial);

const app = new Vue({
  el: '#app',
  components: {Board}
});