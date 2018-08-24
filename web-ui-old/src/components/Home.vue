<template>
  <div class="hello">
    <h1>{{ msg }}</h1>
  </div>

  <img class="logo" src="./../assets/logo.png">
</template>

<script>
export default {
  data () {
    return {
      // note: changing this line won't causes changes
      // with hot-reload because the reloaded component
      // preserves its current state and we are modifying
      // its initial state.
      msg: 'Hello World!'
    }
  }
}
</script>
