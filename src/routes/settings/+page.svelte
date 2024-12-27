<script lang="ts">
    import Navigation from "$lib/Navigation/Navigation.svelte";
    import Main from "$lib/Main.svelte";
    import Title from "$lib/Title.svelte";
    import HugeButton from "$lib/HugeButton.svelte";
    import SettingsItem from "$lib/Settings/SettingsItem.svelte";
    import Category from "$lib/Settings/Category.svelte";
    import Notification from "$lib/Settings/Notification.svelte";

    import { faList, faBell } from "@fortawesome/free-solid-svg-icons";
    import { goto } from "$app/navigation";
    import { fly } from "svelte/transition";

    let showCategory = $state(false);
    let showNotification = $state(false);

    const logout = () => {
        localStorage.clear();
        goto('/login', { replaceState: true });
    }
</script>

<Navigation />
<Main>
    <Title title="Settings" />
    <div class="h-full flex flex-col justify-between items-stretch space-y-2">
        <div class="px-4" in:fly|global={{ x: 50, y: -50 }}>
            {#if localStorage.getItem('role') === 'treasurer'}
            <SettingsItem
                value="Category"
                icon={faList}
                onclick={() => showCategory = true}
            />
            {/if}
        </div>
        <div class="px-4" in:fly|global={{ x: 50, y: -50 }}>
            <SettingsItem
                value="Notification"
                icon={faBell}
                onclick={() => showNotification = true}
            />
        </div>
        <div in:fly|global={{ y: 50 }}>
            <HugeButton value="Logout" onclick={logout} />
        </div>
    </div>
    {#if showCategory}
        <Category bind:showCategory />
    {/if}
    {#if showNotification}
        <Notification bind:showNotification />
    {/if}
</Main>
