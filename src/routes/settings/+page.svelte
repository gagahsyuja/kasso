<script lang="ts">
    import Navigation from "../lib/Navigation/Navigation.svelte";
    import Main from "../lib/Main.svelte";
    import Title from "../lib/Title.svelte";
    import HugeButton from "../lib/HugeButton.svelte";
</script>

<Navigation />
<Main>
    <Title title="Settings" />
    <HugeButton value="Logout" onclick={() => window.location.href = '/login'} />
</Main>
