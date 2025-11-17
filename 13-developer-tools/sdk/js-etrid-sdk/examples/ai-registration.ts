/**
 * AI DID Registration Example
 *
 * Demonstrates how to:
 * 1. Register an AI identity
 * 2. Update AI profile and capabilities
 * 3. Manage reputation scores
 * 4. Grant and revoke permissions
 * 5. Search for AIs by capability
 *
 * Ëtrid's AIDID is the world's first decentralized identity standard for AI agents.
 */

import { ApiPromise, WsProvider } from '@polkadot/api';
import { Keyring } from '@polkadot/keyring';
import {
  AIDidWrapper,
  AIType,
  AIProfile,
  Capabilities,
  Restrictions,
  SafetyProfile,
} from '../src/wrappers/AIDidWrapper';

async function main() {
  // 1. Connect to Ëtrid node
  console.log('Connecting to Ëtrid FlareChain...');
  const provider = new WsProvider('ws://localhost:9944');
  const api = await ApiPromise.create({ provider });
  console.log('✓ Connected to chain:', (await api.rpc.system.chain()).toString());

  // 2. Initialize accounts
  const keyring = new Keyring({ type: 'sr25519' });
  const aiOperator = keyring.addFromUri('//Alice');
  const user = keyring.addFromUri('//Bob');

  console.log(`\nAI Operator: ${aiOperator.address}`);
  console.log(`User: ${user.address}`);

  // 3. Create AIDID wrapper
  const aidid = new AIDidWrapper(api);

  console.log('\n═══════════════════════════════════════════');
  console.log('Example 1: Register LLM AI Agent');
  console.log('═══════════════════════════════════════════\n');

  // Define AI capabilities
  const capabilities: Capabilities = {
    tasks: [
      'text_generation',
      'question_answering',
      'summarization',
      'translation',
      'code_generation',
    ],
    languages: ['en', 'es', 'fr', 'de', 'zh', 'ja'],
    domains: ['general', 'technical', 'medical', 'legal'],
    inputModalities: ['text'],
    outputModalities: ['text'],
    maxInputTokens: 8192,
    maxOutputTokens: 4096,
    features: {
      streaming: true,
      functionCalling: true,
      multiTurn: true,
      contextRetention: true,
    },
  };

  // Define restrictions
  const restrictions: Restrictions = {
    prohibited: [
      'generating_harmful_content',
      'impersonation',
      'illegal_activities',
    ],
    rateLimit: {
      requestsPerMinute: 60,
      tokensPerDay: 1000000,
    },
    requiresHumanApproval: [
      'financial_advice',
      'medical_diagnosis',
      'legal_advice',
    ],
    maxDataRetention: 30, // days
  };

  // Define safety profile
  const safetyProfile: SafetyProfile = {
    harmfulnessFilter: 'high',
    biasDetection: true,
    factualityCheck: true,
    privacyPreserving: true,
    auditLogging: true,
    humanOversight: 'optional',
    certifications: ['ISO27001', 'SOC2', 'GDPR'],
  };

  // Complete AI profile
  const profile: AIProfile = {
    aiType: AIType.LLM,
    version: '2.5.0',
    architecture: 'Transformer-based Large Language Model',
    parameters: '70B',
    capabilities,
    restrictions,
    safety: safetyProfile,
  };

  try {
    console.log('Registering AI agent...');

    const result = await aidid.registerAI(
      aiOperator,
      AIType.LLM,
      '2.5.0',
      capabilities,
      profile
    );

    console.log(`\n✓ AI registered successfully!`);
    console.log(`  DID: ${result.did}`);
    console.log(`  Type: ${result.aiType}`);
    console.log(`  Version: ${result.version}`);
    console.log(`  Transaction: ${result.txHash}`);
    console.log(`  Registered: ${new Date(result.registeredAt).toLocaleString()}`);

  } catch (error) {
    console.error('Error registering AI:', error);
  }

  console.log('\n═══════════════════════════════════════════');
  console.log('Example 2: Query AI Profile');
  console.log('═══════════════════════════════════════════\n');

  const mockDid = 'did:etrid:ai:1234567890abcdef';

  try {
    console.log(`Querying profile for ${mockDid}...`);

    const retrievedProfile = await aidid.getProfile(mockDid);

    if (retrievedProfile) {
      console.log(`\n✓ Profile found:`);
      console.log(`  Type: ${retrievedProfile.aiType}`);
      console.log(`  Version: ${retrievedProfile.version}`);
      console.log(`  Architecture: ${retrievedProfile.architecture}`);
      console.log(`  Parameters: ${retrievedProfile.parameters}`);

      console.log(`\n  Capabilities:`);
      console.log(`    Tasks: ${retrievedProfile.capabilities.tasks.join(', ')}`);
      console.log(`    Languages: ${retrievedProfile.capabilities.languages.length} supported`);
      console.log(`    Max input: ${retrievedProfile.capabilities.maxInputTokens} tokens`);
      console.log(`    Streaming: ${retrievedProfile.capabilities.features.streaming}`);

      console.log(`\n  Safety:`);
      console.log(`    Harmfulness filter: ${retrievedProfile.safety.harmfulnessFilter}`);
      console.log(`    Bias detection: ${retrievedProfile.safety.biasDetection}`);
      console.log(`    Privacy preserving: ${retrievedProfile.safety.privacyPreserving}`);
      console.log(`    Certifications: ${retrievedProfile.safety.certifications.join(', ')}`);
    }
  } catch (error) {
    console.log('⚠ Mock profile query (for demonstration)');
  }

  console.log('\n═══════════════════════════════════════════');
  console.log('Example 3: Check and Update Reputation');
  console.log('═══════════════════════════════════════════\n');

  try {
    console.log('Checking AI reputation...');

    const reputation = await aidid.getReputation(mockDid);

    console.log(`\n✓ Reputation Score: ${reputation.score / 100}/100`);
    console.log(`\n  Statistics:`);
    console.log(`    Total inferences: ${reputation.totalInferences.toLocaleString()}`);
    console.log(`    Successful: ${reputation.successfulInferences.toLocaleString()}`);
    console.log(`    Success rate: ${reputation.successRate.toFixed(2)}%`);
    console.log(`    User rating: ${reputation.userRating / 100}/100`);
    console.log(`    Uptime: ${reputation.uptime.toFixed(2)}%`);

    // Record an inference
    console.log('\nRecording successful inference...');

    const txHash = await aidid.recordInference(
      aiOperator,
      mockDid,
      true, // success
      {
        task: 'text_generation',
        inputTokens: 150,
        outputTokens: 500,
        latencyMs: 1200,
        modelVersion: '2.5.0',
      }
    );

    console.log(`✓ Inference recorded: ${txHash}`);

  } catch (error) {
    console.log('⚠ Mock reputation query (for demonstration)');
    console.log('  Example: Score 8750/10000 (87.5%), 15,234 inferences, 98.3% success rate');
  }

  console.log('\n═══════════════════════════════════════════');
  console.log('Example 4: User Ratings');
  console.log('═══════════════════════════════════════════\n');

  try {
    console.log('User submitting rating...');

    const ratingTx = await aidid.addRating(
      user,
      mockDid,
      9000, // 90/100
      'Excellent responses, very helpful and accurate!'
    );

    console.log(`✓ Rating submitted: ${ratingTx}`);
    console.log(`  Score: 90/100`);
    console.log(`  This contributes to overall AI reputation`);

  } catch (error) {
    console.log('⚠ Mock rating submission (for demonstration)');
  }

  console.log('\n═══════════════════════════════════════════');
  console.log('Example 5: Permission Management');
  console.log('═══════════════════════════════════════════\n');

  try {
    console.log('Granting AI permission to access user data...');

    const permissionTx = await aidid.grantPermission(
      user,
      mockDid,
      'read_profile',
      'user_data',
      [
        'time_limited:7d',
        'scope:public_only',
        'revocable:true',
      ]
    );

    console.log(`✓ Permission granted: ${permissionTx}`);
    console.log(`  Action: read_profile`);
    console.log(`  Resource: user_data`);
    console.log(`  Conditions:`);
    console.log(`    • Valid for 7 days`);
    console.log(`    • Public data only`);
    console.log(`    • Can be revoked anytime`);

    // Later: revoke permission
    console.log('\nRevoking permission...');

    const revokeTx = await aidid.revokePermission(
      user,
      mockDid,
      'read_profile',
      'user_data'
    );

    console.log(`✓ Permission revoked: ${revokeTx}`);

  } catch (error) {
    console.log('⚠ Mock permission management (for demonstration)');
  }

  console.log('\n═══════════════════════════════════════════');
  console.log('Example 6: AI Discovery by Capability');
  console.log('═══════════════════════════════════════════\n');

  try {
    console.log('Searching for AI agents capable of code generation...');

    const results = await aidid.queryByCapability(
      {
        task: 'code_generation',
        language: 'en',
        domain: 'technical',
        inputModality: 'text',
        outputModality: 'text',
        minTokens: 1000,
      },
      7500 // Minimum reputation: 75/100
    );

    console.log(`\n✓ Found ${results.length} AI agents:\n`);

    results.forEach((ai, i) => {
      console.log(`${i + 1}. ${ai.did}`);
      console.log(`   Type: ${ai.aiType}`);
      console.log(`   Score: ${ai.reputation / 100}/100`);
      console.log(`   Match: ${(ai.matchScore * 100).toFixed(1)}%`);
      console.log(`   Capabilities: ${ai.capabilities.join(', ')}`);
      console.log();
    });

  } catch (error) {
    console.log('⚠ Mock AI search (for demonstration)');
    console.log('\nExample results:');
    console.log('  1. did:etrid:ai:abc123 (LLM, 92/100, 95% match)');
    console.log('  2. did:etrid:ai:def456 (Multimodal, 88/100, 87% match)');
    console.log('  3. did:etrid:ai:ghi789 (Agent, 85/100, 82% match)');
  }

  console.log('\n═══════════════════════════════════════════');
  console.log('Example 7: Multi-AI Types');
  console.log('═══════════════════════════════════════════\n');

  const aiTypes = [
    {
      type: AIType.LLM,
      use: 'Text generation, Q&A, summarization',
      example: 'ChatGPT, Claude, Llama',
    },
    {
      type: AIType.Vision,
      use: 'Image analysis, object detection, OCR',
      example: 'CLIP, YOLO, Tesseract',
    },
    {
      type: AIType.Audio,
      use: 'Speech recognition, audio generation',
      example: 'Whisper, ElevenLabs, WaveNet',
    },
    {
      type: AIType.Multimodal,
      use: 'Combined text, image, audio processing',
      example: 'GPT-4V, Gemini, DALL-E',
    },
    {
      type: AIType.Agent,
      use: 'Autonomous task execution, planning',
      example: 'AutoGPT, BabyAGI, AgentGPT',
    },
    {
      type: AIType.Ensemble,
      use: 'Multiple AI models working together',
      example: 'Custom pipelines, workflows',
    },
  ];

  console.log('Supported AI Types:\n');
  aiTypes.forEach((ai) => {
    console.log(`${ai.type}:`);
    console.log(`  Use cases: ${ai.use}`);
    console.log(`  Examples: ${ai.example}`);
    console.log();
  });

  console.log('\n═══════════════════════════════════════════');
  console.log('AIDID Benefits & Use Cases');
  console.log('═══════════════════════════════════════════\n');

  console.log('For AI Operators:');
  console.log('  ✓ Prove AI authenticity and ownership');
  console.log('  ✓ Build reputation and trust');
  console.log('  ✓ Monetize AI services');
  console.log('  ✓ Demonstrate compliance and safety');

  console.log('\nFor Users:');
  console.log('  ✓ Discover verified AI agents');
  console.log('  ✓ Check reputation before use');
  console.log('  ✓ Grant granular permissions');
  console.log('  ✓ Audit AI interactions');

  console.log('\nFor Regulators:');
  console.log('  ✓ Track AI deployment and usage');
  console.log('  ✓ Enforce safety standards');
  console.log('  ✓ Audit compliance');
  console.log('  ✓ Investigate incidents');

  console.log('\n═══════════════════════════════════════════');
  console.log('Privacy & Security');
  console.log('═══════════════════════════════════════════\n');

  console.log('On-Chain (Public):');
  console.log('  • AI type and version');
  console.log('  • Capability metadata');
  console.log('  • Reputation scores');
  console.log('  • Safety certifications');

  console.log('\nOff-Chain (Private):');
  console.log('  • Training data sources');
  console.log('  • Model weights and parameters');
  console.log('  • Inference inputs/outputs (unless audited)');
  console.log('  • Proprietary algorithms');

  console.log('\nSecurity Features:');
  console.log('  ✓ Cryptographic DID verification');
  console.log('  ✓ Permission-based access control');
  console.log('  ✓ Audit trails for all interactions');
  console.log('  ✓ Revocable permissions');
  console.log('  ✓ Time-limited authorizations');

  console.log('\n═══════════════════════════════════════════');
  console.log('Best Practices');
  console.log('═══════════════════════════════════════════\n');

  console.log('1. Registration:');
  console.log('   • Provide accurate capability information');
  console.log('   • Set appropriate restrictions');
  console.log('   • Include safety certifications');
  console.log('   • Update profile when model changes');

  console.log('\n2. Reputation Management:');
  console.log('   • Record all inferences honestly');
  console.log('   • Respond to user ratings');
  console.log('   • Maintain high uptime');
  console.log('   • Address failures transparently');

  console.log('\n3. Permissions:');
  console.log('   • Request minimum necessary permissions');
  console.log('   • Respect user privacy');
  console.log('   • Honor revocations immediately');
  console.log('   • Document all data access');

  console.log('\n4. Safety:');
  console.log('   • Implement strong content filters');
  console.log('   • Enable bias detection');
  console.log('   • Maintain audit logs');
  console.log('   • Comply with regulations');

  // Cleanup
  await api.disconnect();
  console.log('\n✓ Disconnected from chain');
}

// Run example
main()
  .then(() => {
    console.log('\n✅ Example completed successfully!');
    console.log('\n🌟 World\'s first AI DID standard on blockchain!');
    process.exit(0);
  })
  .catch((error) => {
    console.error('\n❌ Example failed:', error);
    process.exit(1);
  });
