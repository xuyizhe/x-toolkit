<script>
  import x_toolkit_convert_wasm from "@x-toolkit/convert-wasm";
  import x_toolkit_crypto_wasm from "@x-toolkit/crypto-wasm";

  export const HELLO_WORLD = "hello, world";

  export let name;
  export let x_toolkit_convert;
  export let x_toolkit_crypto;
  export let convert_string_from_bytes_result;
  export let convert_string_to_bytes_result;
  export let convert_hex_encode_result;
  export let convert_hex_decode_result;
  export let crypto_digest_sha256_result;
  export let crypto_digest_ripemd160_result;

  async function loadWasm() {
    x_toolkit_convert = await x_toolkit_convert_wasm();
    x_toolkit_crypto = await x_toolkit_crypto_wasm();
    convert_string_to_bytes_result = x_toolkit_convert.string_to_bytes(HELLO_WORLD);
    convert_string_from_bytes_result = x_toolkit_convert.string_from_bytes(
      convert_string_to_bytes_result
    );
    convert_hex_encode_result = x_toolkit_convert.hex_encode(convert_string_to_bytes_result);
    convert_hex_decode_result = x_toolkit_convert.hex_decode(convert_hex_encode_result);
    crypto_digest_sha256_result = x_toolkit_crypto.digest_sha256(convert_string_to_bytes_result);
    crypto_digest_ripemd160_result = x_toolkit_crypto.digest_ripemd160(
      convert_string_to_bytes_result
    );
  }

  loadWasm();
</script>

<style>
  main {
    padding: 1em;
    margin: 0 auto;
  }

  h1 {
    color: #ff3e00;
    text-transform: uppercase;
    text-align: center;
    font-size: 4em;
    font-weight: 100;
  }

  .container {
    width: 750px;
    margin: 0 auto;
    text-align: left;
    word-wrap: break-word;
  }

  @media (max-width: 750px) {
    .container {
      width: 100%;
    }
  }
</style>

<main>
  <h1>Hello {name}!</h1>
  <!-- <p>Visit the <a href="https://svelte.dev/tutorial">Svelte tutorial</a> to learn how to build Svelte apps.</p> -->

  <div class="container">
    <section>
      <h2>Convert</h2>
      <details open>
        <summary>x_toolkit_convert.string_to_bytes("{HELLO_WORLD}")</summary>
        <code>Uint8Array(12) [{convert_string_to_bytes_result}]</code>
      </details>
      <br />
      <details open>
        <summary>
          x_toolkit_convert.string_from_bytes(new Uint8Array([{convert_string_to_bytes_result}]))
        </summary>
        <code>{convert_string_from_bytes_result}</code>
      </details>
      <br />
      <details open>
        <summary>
          x_toolkit_convert.hex_encode(new Uint8Array([{convert_string_to_bytes_result}]))
        </summary>
        <code>{convert_hex_encode_result}</code>
      </details>
      <br />
      <details open>
        <summary>x_toolkit_convert.hex_decode("{convert_hex_encode_result}")</summary>
        <code>Uint8Array(12) [{convert_hex_decode_result}]</code>
      </details>
    </section>

    <section>
      <h2>Crypto</h2>
      <details open>
        <summary>
          x_toolkit_crypto.digest_sha256(new Uint8Array([{convert_string_to_bytes_result}]))
        </summary>
        <code>Uint8Array(32) [{crypto_digest_sha256_result}]</code>
      </details>
      <br />
      <details open>
        <summary>
          x_toolkit_crypto.digest_ripemd160(new Uint8Array([{convert_string_to_bytes_result}]))
        </summary>
        <code>Uint8Array(20) [{crypto_digest_ripemd160_result}]</code>
      </details>
    </section>
  </div>
</main>
