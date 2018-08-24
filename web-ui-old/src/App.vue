<template>
  <div id="app">
    <router-view :dm.sync="dm"></router-view>

    <menu></menu>
    <navigation></navigation>
  </div>
</template>

<script>
import Navigation from './components/Navigation'
import Menu from './components/Menu'
import './ws'
import './assets/roboto/Roboto-Bold.ttf'
import './assets/roboto/Roboto-Medium.ttf'
import './assets/roboto/Roboto-Thin.ttf'

export default {
  components: {
    Navigation,
    Menu
  },
  data () {
    return {
      dm: new window.Devicemanager([])
    }
  },
  ready: function () {
  }
}
</script>

<style>
body {
  margin: 0px;
  margin-top: 40px;
  font-family: 'Roboto', sans-serif;
  background: #00EBC1;
}
</style>
