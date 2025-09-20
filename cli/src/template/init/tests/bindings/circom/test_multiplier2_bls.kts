import uniffi.mopro.*

try {
    var zkeyPath = "./test-vectors/circom/multiplier2_bls_final.zkey"

    val input_str: String = "{\"b\":[\"5\"],\"a\":[\"3\"]}"

    // Generate proof
    var generateProofResult = generateCircomProof(zkeyPath, input_str, ProofLib.ARKWORKS)

    // Verify proof
    var isValid = verifyCircomProof(zkeyPath, generateProofResult, ProofLib.ARKWORKS)
    assert(isValid) { "Proof is invalid" }


} catch (e: Exception) {
    println(e)
    throw e
}
