<template>
<div>
  <md-layout v-for="(row, i) in parseInt(height)" class="row">
    <md-layout v-for="(tile, j) in parseInt(width)" md-gutter md-align="center">
      <md-card>
        <md-card-content>
          {{i}} {{j}}
        </md-card-content>
      </md-card>
    </md-layout>
  </md-layout>
</div>
</template>
<script>
export default {
  data() {
    return {};
  },
  props: ['width', 'height'],
  created() {
    console.log('Board initialized', this.width, this.height);
  },
  methods: {}
};
</script>
<style>
.row {
  padding-top: 12px;
}
</style>