<script>
  import XToolkitConvertWasm from "@x-toolkit/convert-wasm";
  import XToolkitCryptoWasm from "@x-toolkit/crypto-wasm";

  export let name;
  export let convert_string_to_bytes_result;
  export let convert_bytes_to_string_result;
  export let convert_hex_encode_result;
  export let convert_hex_decode_result;
  export let crypto_digest_sha256_result;

  async function loadConvertWasm() {
    const exports = await XToolkitConvertWasm();
    convert_string_to_bytes_result = exports.convert_string_to_bytes("hello, world");
    convert_bytes_to_string_result = exports.convert_bytes_to_string([
      104,
      101,
      108,
      108,
      111,
      44,
      32,
      119,
      111,
      114,
      108,
      100,
    ]);
    convert_hex_encode_result = exports.convert_hex_encode(convert_string_to_bytes_result);
    convert_hex_decode_result = exports.convert_hex_decode(convert_hex_encode_result);
  }

  async function loadCryptoWasm() {
    const exports = await XToolkitCryptoWasm();
    crypto_digest_sha256_result = exports.crypto_digest_sha256(convert_string_to_bytes_result);
  }

  loadConvertWasm();
  loadCryptoWasm();
</script>

<style>
  main {
    text-align: center;
    padding: 1em;
    max-width: 240px;
    margin: 0 auto;
  }

  h1 {
    color: #ff3e00;
    text-transform: uppercase;
    font-size: 4em;
    font-weight: 100;
  }

  .container {
    width: 600px;
    margin: 0 auto;
    padding: 20px;
    text-align: left;
  }

  @media (min-width: 640px) {
    main {
      max-width: none;
    }
  }
</style>

<main>
  <h1>Hello {name}!</h1>
  <!-- <p>Visit the <a href="https://svelte.dev/tutorial">Svelte tutorial</a> to learn how to build Svelte apps.</p> -->

  <div class="container">
    <section>
      <h2>Convert</h2>
      <details>
        <summary>convert_string_to_bytes("hello, world")</summary>
        {convert_string_to_bytes_result}
      </details>
      <br />
      <details>
        <summary>convert_bytes_to_string([104,101,108,108,111,44,32,119,111,114,108,100])</summary>
        {convert_bytes_to_string_result}
      </details>
      <br />
      <details>
        <summary>convert_hex_encode([104,101,108,108,111,44,32,119,111,114,108,100])</summary>
        {convert_hex_encode_result}
      </details>
      <br />
      <details>
        <summary>convert_hex_encode("68656c6c6f2c20776f726c64")</summary>
        {convert_hex_decode_result}
      </details>
    </section>

    <section>
      <h2>Crypto</h2>
      <details>
        <summary>
          crypto_digest_sha256(new Uint8Array([104, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108,
          100]))
        </summary>
        {crypto_digest_sha256_result}
      </details>
    </section>
  </div>
</main>
