<script lang="ts">
    import Fa from "svelte-fa";
    import Database from "@tauri-apps/plugin-sql";
    import Input from "$lib/Input.svelte";
    import { faUser, faLock, faKey } from "@fortawesome/free-solid-svg-icons";
    import { scale } from "svelte/transition";
    import { goto } from "$app/navigation";
    import { sha256 } from "js-sha256";

    let username = $state('');
    let password = $state('');
    let confirmPassword = $state('');
    let code = $state('');
    let message = $state('');
    let role = $state('member');

    const checkPassword = () => {

        if (password !== confirmPassword) {
            message = 'Password does not match!';
            return false;
        }

        return true;
    };

    const checkUsername = async () => {

        const db = await Database.load("sqlite:database.db");

        let result: any = await db.select("SELECT * FROM users WHERE username = $1", [username]);

        if (result.length) {
            message = 'Username already exist!';
            return false;
        }

        return true;
    };

    const checkCode = () => {
        if (code === '999') role = 'treasurer';
    };

    const handleRegister = async () => {

        if (checkPassword() && await checkUsername()) {

            let hashedPassword = sha256(password);

            checkCode();

            const db = await Database.load("sqlite:database.db");

            let result = await db.execute(
                "INSERT INTO users (role, username, password, notify) VALUES ($1, $2, $3, $4)", 
                [ role, username, hashedPassword, 500000 ]
            );

            message = '';
            localStorage.setItem('userId', result.lastInsertId!.toString());
            localStorage.setItem('username', username)
            localStorage.setItem('role', role)
            localStorage.setItem('notify', '500000')
            goto('/', { replaceState: true });
        }
    };
</script>

<div class="fixed h-full w-full flex flex-col justify-evenly items-center">
    <h1 class="text-6xl font-extrabold text-blue-900">
        KASSO
    </h1>
    <div class="flex flex-col w-full justify-center items-center px-8 space-y-3">
        {#key message}
            <div class="{message !== '' ? "opacity-100" : "opacity-0" } rounded-xl w-full text-center
                font-bold px-6 py-2 bg-red-500 text-white"
                in:scale|global={{ duration: 100 }}
            >
                {message}
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
        <Input
            icon={faLock}
            name={"Confirm Password"}
            type={"password"}
            bind:value={confirmPassword}
        />
        <Input
            icon={faKey}
            name={"Secret Key (Optional)"}
            type={"password"}
            bind:value={code}
        />
        <div class="flex flex-col justify-center items-center w-full py-4 space-y-2">
            <button onclick="{handleRegister}" class="bg-blue-900 text-white font-semibold p-4 w-full rounded-[30px]">Register</button>
            <a href="/login">Login</a>
        </div>
    </div>
</div>

<style>
    :global(html) {
        background-color: rgb(239 246 255);
    }
</style>
