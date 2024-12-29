<script lang="ts">
    import Fa from "svelte-fa";
    import Database from "@tauri-apps/plugin-sql";
    import Input from "$lib/Input.svelte";
    import { faUser, faLock } from "@fortawesome/free-solid-svg-icons";
    import { goto } from "$app/navigation";
    import { onMount } from "svelte";
    import { scale } from "svelte/transition";
    import { sha256, type Message } from "js-sha256";

    let username = $state();
    let password = $state();
    let wrongCredential = $state(false);

    const checkPassword = async () => {

        let hashedPassword = sha256(password as Message);

        const db = await Database.load("sqlite:database.db");

        let result: any = await db.select("SELECT * FROM users\
            WHERE username = $1 AND password = $2 LIMIT 1",
            [username, hashedPassword]);

        if (result.length) {
            localStorage.setItem('username', result[0].username)
            localStorage.setItem('role', result[0].role)
            localStorage.setItem('notify', result[0].notify)
            goto('/', { replaceState: true })
            wrongCredential = false;
        } else {
            wrongCredential = true;
        }
    };

    onMount(() => {
        if (localStorage.getItem('username')) goto('/', { replaceState: true })
    })
</script>

<div class="fixed flex flex-col justify-evenly items-center w-full h-full">
    <h1 class="text-6xl font-extrabold text-blue-900">
        KASSO
    </h1>
    <div class="flex flex-col w-full justify-center items-center px-8 space-y-3">
        {#key wrongCredential}
            <div class="{wrongCredential ? "opacity-100" : "opacity-0" } rounded-xl w-full text-center
                font-bold px-6 py-2 bg-red-500 text-white"
                in:scale|global={{ duration: 100 }}
            >
                Wrong username or password!
            </div>
        {/key}
        <Input
            icon={faUser}
            name={"Username"}
            type={"text"}
            bind:value={username}
        />
        <Input
            icon={faLock}
            name={"Password"}
            type={"password"}
            bind:value={password}
        />
        <div class="flex flex-col justify-center items-center w-full py-7 space-y-2">
            <button onclick="{checkPassword}" class="bg-blue-900 text-white font-semibold p-4 w-full rounded-[30px]">Login</button>
            <a href="/register">Create Account</a>
        </div>
    </div>
</div>

<style>
    :global(html) {
        background-color: rgb(239 246 255);
    }
</style>
