<template>
  <navigation-side :dm="dm" :ll="ll" :active.sync="active"></navigation-side>
  <div class="settings">


    <span>Ich muss sichtbar sein....{{active}}</span>
  </div>
</template>

<script>
import NavigationSide from './NavigationSide'

export default {
  components: {
    NavigationSide
  },
  data () {
    return {
      ll: [
        {name: 'Hallo', logo: 'cube'},
        {name: 'Test', logo: 'cube'}
      ],
      active: null
    }
  }
}
</script>

<style>
.settings {
  margin-left: 250px;
}

@media (max-width: 519px) {
  .settings {
    margin-left: 0px;
  }
}
</style>
