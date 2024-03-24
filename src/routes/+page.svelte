<script>
    import { goto } from "$app/navigation";
    import { redirect } from "@sveltejs/kit";
    import { invoke } from "@tauri-apps/api/tauri";

    let path = "C:\\Users\\elieu\\AppData\\Local\\rustKey\\master.json";

    let master_password = "";

    async function master_password_existence() {
        return await invoke("is_file_here", { path });
    }

    async function register() {
        await invoke("register_master_password", { master_password });
    }

    async function password_check() {
        console.log("master password:", master_password);
        return await invoke("verify_master_password", { master_password });
    }

    async function login() {
        let pass = await password_check();
        console.log("check_password:", pass);

        if (pass) {
            console.log("CORRECT PASSWORD !!");
            goto("/logins");
        } else {
            console.log("WRONG PASSWORD !!");
        }
    }
</script>

<div class="log_screen">
    <div class="left" />
    <div class="input_pass">
        <input
            placeholder="Password"
            type="password"
            bind:value={master_password}
        />
    </div>
    <div class="button_part">
        <div class="button_pass">
            {#await master_password_existence() then is_here}
                {#if is_here}
                    <button on:click={() => login()}>login</button>
                {:else}
                    <button on:click={register}>register</button>
                {/if}
            {/await}
        </div>
        <div class="logo">
            <img src="" alt="" />
        </div>
    </div>
</div>

<style>
    div.log_screen {
        display: flex;
        width: 100%;
        height: 100%;

        background-color: #fefded;
    }

    .left {
        width: 30vw;
        border-right: solid 1px #fa7070;
        margin-right: 3rem;
    }

    .input_pass {
        width: 20rem;
        margin-right: 1rem;

        display: flex;
        place-content: center center;
    }

    input {
        box-sizing: border-box;
        position: absolute;
        background: #ffffff;
        border-radius: 0.4rem;
        width: 20rem;
        height: 2rem;

        outline: none;
        border: solid 1px #a1c398;
        background-color: #eeeddd;
    }

    input:focus {
        background-color: #fefded;
        border: solid 1px #fa7070;
    }

    .button_part {
        width: 5rem;
    }

    button {
        width: 5rem;
        height: 2rem;
    }
</style>
