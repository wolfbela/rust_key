<script>
    import { invoke } from "@tauri-apps/api/tauri";

    let path = "C:\\Users\\elieu\\AppData\\Local\\rustKey.json";

    let name = "";
    let greetMsg = "";
    let master_password = "";
    let login_promise = login();

    async function master_password_existence() {
        return await invoke("is_file_here", { path });
    }

    async function greet() {
        greetMsg = await invoke("greet", { name });
    }

    async function register() {
        await invoke("register_master_password", { master_password });
    }

    async function login() {
        console.log(master_password);
        return await invoke("verify_master_password", { master_password });
    }
</script>

<div class="log_screen">
    <div class="input_pass">
        <input
            id="greet-input"
            placeholder="Enter a name..."
            bind:value={master_password}
            class="input_pass"
        />
    </div>
    <div class="button_part">
        <div class="button_pass">
            {#await master_password_existence() then is_here}
                {#if is_here}
                    <button on:click={() => (login_promise = login())}
                        >login</button
                    >
                    {#await login_promise then is_logged}
                        {#if is_logged}
                            <p>YOU'RE LOGGED IN !</p>
                        {:else}
                            <p>WRONG PASSWORD</p>
                        {/if}
                    {/await}
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
        width: 100%;
        height: 100%;
        display: flex;
        flex-direction: row;
        justify-content: center;
        align-items: center;
    }

    div.input_pass {
        flex: 1;
        display: flex;
        flex-direction: row;
        justify-content: center;
        align-items: center;
    }

    input.input_pass {
        box-sizing: border-box;
        position: absolute;
        width: 10cm;
        height: 1cm;
        background: #ffffff;
        border-radius: 9px;
    }

    div.button_part {
        flex: 1;
        display: flex;
        flex-direction: row;
    }

    div.button_pass {
        flex: 1;
        padding: 10px;
    }

    div.button_pass > button {
        width: 5cm;
        height: 1cm;
    }
</style>
