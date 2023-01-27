<script>
	import { fns, AppShell, Navbar, } from '@svelteuidev/core';
	import NavContent from './NavContent.svelte';

	let isDark = false;
	let opened = false;

	function toggleTheme() {
		isDark = !isDark;
	}
	function toggleOpened() {
		opened = !opened;
	}
</script>

<AppShell>
	<Navbar slot="navbar" hidden={!opened}
	width={{
        // When viewport is larger than theme.breakpoints.sm, Navbar width will be 300px
        sm: 300,

        // When viewport is larger than theme.breakpoints.lg, Navbar width will be 400px
        lg: 400,

        // When other breakpoints do not match base width is used, defaults to 100%
        base: 100
    }}>
		
		<NavContent 
		/>
	</Navbar>

	<slot>This is the main content</slot>
</AppShell>