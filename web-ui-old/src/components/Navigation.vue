<template>
  <div class="nav">
    <ul>
      <li><a v-on:click="toggleMenu()" v-link="{ path: '/' }"><span class="icon"><i class="fa fa-home"></i></span><span>Home</span></a></li>
      <li><a v-on:click="toggleMenu()" v-link="{ path: '/all-devices' }"><span class="icon"><i class="fa fa-fort-awesome"></i></span><span>Other</span></a></li>
      <li><a v-on:click="toggleMenu()" v-link="{ path: '/all-devices' }"><span class="icon"><i class="fa fa-cube"></i></span><span>Devices</span></a></li>
      <li><a v-on:click="toggleMenu()" v-link="{ path: '/settings' }"><span class="icon"><i class="fa fa-cog"></i></span><span>Settings</span></a></li>
    </ul>
  </div>
</template>

<script>
export default {
  data () {
    return {
    }
  },
  ready: function () {
    this.setItemWidth()
  },
  methods: {
    // Set the with of each point depending of the amount
    // of points
    setItemWidth: function () {
      $('.nav >> li').css('width', (100 / $('.nav >> li').length) + '%')
    },
    toggleMenu: function () {
      $('.nav').toggleClass('mshow')
      $('.menu-toggle').toggleClass('open')
    }
  }
}
</script>

<style>
/* Global CSS that are applied for all screen sizes */
.nav ul {
  position: fixed;
  bottom: 0px;
  width: 100%;
	margin: 0;
	padding: 0;
	list-style: none;
	font-size: 1.5em;
	font-weight: 300;
  z-index: 2;
}

.nav a {
	display: block;
	color: rgba(249, 249, 249, .9);
	text-decoration: none;
	transition: color .5s, background .5s, margin-top .5s, height .5s;
}

/* Make the font smoother for Chrome */
.nav i{
	transform: translate3d(0, 0, 0);
}

/* Remove the blue Webkit background when element is tapped */
.nav a, button {
	-webkit-tap-highlight-color: rgba(0,0,0,0);
}

/* Hover effect for the whole navigation to make the hovered item stand out */
.nav ul:hover a {
	color: rgba(249, 249, 249, .5);
}

.nav ul:hover a:hover {
	color: rgba(249, 249, 249, 0.99);
}

/* color of the menu points */
.nav a {
	background: #333333;
}

/* display at screen width bigger as 800px */
@media (min-width: 520px) {

  .nav li span {
  	display: block;
  }

	/* Transforms the list into a horizontal navigation */
	.nav li {
		float: left;
		text-align: center;
		transition: border .5s;
	}

	.nav a {
		display: block;
		width: auto;
    padding-bottom: 10px;
    font-size: 0.8em;
	}

  /* hover, focused and active effects that add a little colored border to the different items */
  .nav li a:hover {
		border-bottom: 10px solid rgb(16, 34, 44);
	}

  /* Placing the icon */
  .nav .icon {
    padding-top: 1em;
  }

  .nav .icon + span {
    visibility: hidden;
  }

  /* Animating the height of the element*/
	.nav a {
		height: 5em;
    margin-top: 1em;
	}

	.nav a:hover {
		height: 6em;
    margin-top: 0em;
    padding-bottom: 0px;
	}

  /* Positioning the icons and preparing for the animation*/
	.nav i {
		position: relative;
		display: inline-block;
		margin: 0 auto;
		padding: 0.4em;
		border-radius: 50%;
		font-size: 1.8em;
		box-shadow: 0 0 0 0.8em transparent;
		background: rgba(255,255,255,0.1);
		transform: translate3d(0, 0, 0);
		transition: box-shadow .6s ease-in-out;
	}

  /* Animate the box-shadow to create the effect */
  .nav a:hover i {
    box-shadow: 0 0 0px 0px rgba(255,255,255,0.2);
    transition: box-shadow .4s ease-in-out;
  }
}

/* display at screen width bigger as 800px */
@media (min-width: 800px) {
  /* Placing the icon */
  .nav .icon {
    padding-top: 1em;
  }

  .nav .icon + span {
    margin-top: 0.7em;
    transition: margin .5s;
    visibility: visible;
  }

  /* Animating the height of the element*/
  .nav a {
    height: 6em;
    margin-top: 1em;
    font-size: 1em;
  }

  .nav a:hover {
    height: 7em;
    margin-top: 0em;
    padding-bottom: 0px;
  }
}

/* display at screen smaller as 520px */
@media (max-width: 519px) {
  .nav li {
    width: 100% !important;
  }

  .nav {
    display: none;
  }

  .nav.mshow {
    display: block;
  }

  .nav ul {
    top: 40px;
    width: 250px;
    overflow: hidden;
  }

  .nav a {
    padding-top: 5px;
    padding-bottom: 5px;
  }

  .nav .icon {
    margin-left: 10px;
  }

  .nav .icon + span {
    margin-left: 15px;
  }

  .nav li a:hover {
    border-right: 10px solid rgb(16, 34, 44);
  }
}
</style>
