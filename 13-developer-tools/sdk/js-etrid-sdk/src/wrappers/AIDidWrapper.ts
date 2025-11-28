/**
 * AIDID (AI Decentralized Identity) Wrapper for Ëtrid SDK
 *
 * World's first AI DID standard for registering, managing, and discovering
 * AI entities on-chain with reputation tracking and permission management.
 */

import { ApiPromise } from '@polkadot/api';
import type { KeyringPair } from '@polkadot/keyring/types';
import { ValidationError, TransactionError } from '../errors/EtridErrors';

/**
 * AI Decentralized Identity (unique identifier)
 */
export type AIDID = string;

/**
 * AI type classification
 */
export enum AIType {
  /** Large Language Model */
  LLM = 'LLM',
  /** Computer Vision Model */
  Vision = 'Vision',
  /** Audio Processing Model */
  Audio = 'Audio',
  /** Multi-modal AI System */
  Multimodal = 'Multimodal',
  /** Autonomous AI Agent */
  Agent = 'Agent',
  /** Ensemble of Multiple Models */
  Ensemble = 'Ensemble',
}

/**
 * AI task categories
 */
export enum Task {
  TextGeneration = 'TextGeneration',
  TextClassification = 'TextClassification',
  QuestionAnswering = 'QuestionAnswering',
  CodeGeneration = 'CodeGeneration',
  Translation = 'Translation',
  Summarization = 'Summarization',
  ImageGeneration = 'ImageGeneration',
  ImageClassification = 'ImageClassification',
  ObjectDetection = 'ObjectDetection',
  AudioTranscription = 'AudioTranscription',
  AudioGeneration = 'AudioGeneration',
  VideoGeneration = 'VideoGeneration',
  VideoAnalysis = 'VideoAnalysis',
  Reasoning = 'Reasoning',
  PlanningExecution = 'PlanningExecution',
  DataAnalysis = 'DataAnalysis',
}

/**
 * Input/Output modality
 */
export enum Modality {
  Text = 'Text',
  Image = 'Image',
  Audio = 'Audio',
  Video = 'Video',
  StructuredData = 'StructuredData',
  Code = 'Code',
}

/**
 * AI capabilities
 */
export interface Capabilities {
  /** Tasks this AI can perform */
  tasks: Task[];
  /** Supported input modalities */
  inputModalities: Modality[];
  /** Supported output modalities */
  outputModalities: Modality[];
  /** Supported languages (ISO 639-1 codes) */
  languages: string[];
  /** Maximum context window (tokens) */
  maxContext?: number;
  /** Maximum output tokens */
  maxOutput?: number;
}

/**
 * AI restrictions
 */
export interface Restrictions {
  /** Tasks this AI cannot perform */
  prohibitedTasks: Task[];
  /** Content filtering enabled */
  contentFiltering: boolean;
  /** Requires human oversight */
  requiresSupervision: boolean;
  /** Cannot access real-time data */
  noRealtimeData: boolean;
  /** Knowledge cutoff date (UNIX timestamp) */
  knowledgeCutoff?: number;
  /** Maximum rate limit (requests per second) */
  rateLimit?: number;
}

/**
 * Safety profile
 */
export interface SafetyProfile {
  /** Alignment method (e.g., "RLHF", "Constitutional AI") */
  alignmentMethod: string;
  /** Content filtering enabled */
  contentFiltering: boolean;
  /** Bias evaluation performed */
  biasEvaluated: boolean;
  /** Toxicity score (0-10000 representing 0.00% - 100.00%) */
  toxicityScore: number;
}

/**
 * Benchmark result
 */
export interface Benchmark {
  /** Benchmark name (e.g., "MMLU", "HumanEval") */
  name: string;
  /** Score (0-10000 representing 0.00% - 100.00%) */
  score: number;
}

/**
 * Model attestation
 */
export interface ModelAttestation {
  /** Model hash (SHA-256 of weights) */
  modelHash: string;
  /** Training data hash (IPFS CID or similar) */
  trainingDataHash: string;
  /** Model version */
  version: string;
  /** Training timestamp (UNIX) */
  trainingDate: number;
  /** Reproducible build */
  reproducible: boolean;
  /** Benchmark scores */
  benchmarks: Benchmark[];
}

/**
 * Complete AI profile
 */
export interface AIProfile {
  /** AI type */
  aiType: AIType;
  /** Model version */
  version: string;
  /** Model architecture description */
  architecture: string;
  /** Number of parameters (as string for very large numbers) */
  parameters: string;
  /** Capabilities */
  capabilities: Capabilities;
  /** Restrictions */
  restrictions: Restrictions;
  /** Safety features */
  safety: SafetyProfile;
  /** Model attestation (optional) */
  attestation?: ModelAttestation;
}

/**
 * Reputation score
 */
export interface Reputation {
  /** Overall score (0-10000 representing 0.00 - 100.00) */
  score: number;
  /** Total number of inferences performed */
  totalInferences: number;
  /** Successful inferences */
  successfulInferences: number;
  /** Failed inferences */
  failedInferences: number;
  /** User ratings (0-10000) */
  userRating: number;
  /** Number of user ratings */
  ratingCount: number;
  /** Uptime percentage (0-10000) */
  uptime: number;
  /** Number of incidents reported */
  incidents: number;
  /** Success rate percentage */
  successRate: number;
}

/**
 * Authorization permission
 */
export interface Permission {
  /** Permission ID */
  id: string;
  /** Action this AI is allowed to perform */
  action: string;
  /** Resource this applies to */
  resource: string;
  /** Conditions that must be met */
  conditions: string[];
  /** When permission was granted */
  grantedAt: number;
  /** Who granted the permission */
  grantedBy: string;
}

/**
 * Inference metadata
 */
export interface InferenceMetadata {
  /** Task performed */
  task: Task;
  /** Input size (tokens/bytes) */
  inputSize: number;
  /** Output size (tokens/bytes) */
  outputSize: number;
  /** Processing time (milliseconds) */
  processingTime: number;
  /** Cost (if applicable) */
  cost?: bigint;
}

/**
 * AI registration result
 */
export interface AIRegistrationResult {
  /** Generated AIDID */
  did: AIDID;
  /** Transaction hash */
  txHash: string;
  /** Registration block */
  blockNumber: number;
}

/**
 * AI search result
 */
export interface AISearchResult {
  /** AI DID */
  did: AIDID;
  /** AI profile */
  profile: AIProfile;
  /** Reputation score */
  reputation: Reputation;
  /** Match score (0-100) */
  matchScore: number;
}

/**
 * AIDID wrapper for AI identity management
 *
 * Enables registration and management of AI identities on-chain,
 * including reputation tracking, capability declarations, and
 * permission management.
 *
 * @example
 * ```typescript
 * const aidid = new AIDidWrapper(api);
 *
 * // Register an AI
 * const result = await aidid.registerAI(
 *   alice,
 *   AIType.LLM,
 *   '1.0.0',
 *   capabilities,
 *   profile
 * );
 *
 * // Get AI profile
 * const profile = await aidid.getProfile(result.did);
 *
 * // Record inference
 * await aidid.recordInference(alice, result.did, true);
 * ```
 */
export class AIDidWrapper {
  constructor(private api: ApiPromise) {}

  /**
   * Registers a new AI identity
   *
   * Creates a new on-chain AI DID with complete profile information.
   *
   * @param signer - Your account (must be AI operator)
   * @param aiType - Type of AI (LLM, Vision, etc.)
   * @param version - Model version
   * @param capabilities - AI capabilities
   * @param profile - Complete AI profile
   * @returns Promise resolving to registration result
   *
   * @throws {ValidationError} If profile is invalid
   * @throws {TransactionError} If registration fails
   *
   * @example
   * ```typescript
   * const capabilities: Capabilities = {
   *   tasks: [Task.TextGeneration, Task.QuestionAnswering],
   *   inputModalities: [Modality.Text],
   *   outputModalities: [Modality.Text],
   *   languages: ['en', 'es', 'fr'],
   *   maxContext: 8192,
   *   maxOutput: 4096
   * };
   *
   * const profile: AIProfile = {
   *   aiType: AIType.LLM,
   *   version: '1.0.0',
   *   architecture: 'Transformer',
   *   parameters: '7B',
   *   capabilities,
   *   restrictions: { /* ... */ },
   *   safety: { /* ... */ }
   * };
   *
   * const result = await aidid.registerAI(alice, AIType.LLM, '1.0.0', capabilities, profile);
   * console.log('AI registered:', result.did);
   * ```
   */
  async registerAI(
    signer: KeyringPair,
    aiType: AIType,
    version: string,
    capabilities: Capabilities,
    profile: AIProfile
  ): Promise<AIRegistrationResult> {
    try {
      // Validate profile
      this.validateProfile(profile);

      return new Promise((resolve, reject) => {
        this.api.tx.aidid
          .registerAI(
            aiType,
            version,
            profile.architecture,
            profile.parameters,
            this.encodeCapabilities(capabilities),
            this.encodeRestrictions(profile.restrictions),
            this.encodeSafety(profile.safety)
          )
          .signAndSend(signer, ({ status, events, dispatchError }) => {
            if (status.isFinalized) {
              if (dispatchError) {
                if (dispatchError.isModule) {
                  const decoded = this.api.registry.findMetaError(dispatchError.asModule);
                  reject(new TransactionError(`${decoded.section}.${decoded.name}: ${decoded.docs.join(' ')}`));
                } else {
                  reject(new TransactionError(dispatchError.toString()));
                }
              } else {
                // Find AIRegistered event
                const registerEvent = events.find(({ event }) =>
                  this.api.events.aidid.AIRegistered.is(event)
                );

                if (registerEvent) {
                  const [did, owner] = registerEvent.event.data;
                  resolve({
                    did: did.toString(),
                    txHash: status.asFinalized.toHex(),
                    blockNumber: status.asFinalized.toNumber(),
                  });
                } else {
                  reject(new TransactionError('AIRegistered event not found'));
                }
              }
            }
          });
      });
    } catch (error) {
      throw new TransactionError(`Failed to register AI: ${error.message}`);
    }
  }

  /**
   * Updates an AI profile
   *
   * Updates metadata for an existing AI DID.
   *
   * @param signer - AI owner account
   * @param did - AI DID to update
   * @param updates - Profile updates (partial)
   * @returns Promise resolving to transaction hash
   *
   * @throws {TransactionError} If update fails
   *
   * @example
   * ```typescript
   * await aidid.updateProfile(alice, aiDid, {
   *   version: '1.1.0',
   *   capabilities: updatedCapabilities
   * });
   * ```
   */
  async updateProfile(
    signer: KeyringPair,
    did: AIDID,
    updates: Partial<AIProfile>
  ): Promise<string> {
    try {
      return new Promise((resolve, reject) => {
        this.api.tx.aidid
          .updateProfile(
            did,
            updates.version,
            updates.architecture,
            updates.parameters,
            updates.capabilities ? this.encodeCapabilities(updates.capabilities) : null,
            updates.restrictions ? this.encodeRestrictions(updates.restrictions) : null,
            updates.safety ? this.encodeSafety(updates.safety) : null
          )
          .signAndSend(signer, ({ status, dispatchError }) => {
            if (status.isFinalized) {
              if (dispatchError) {
                if (dispatchError.isModule) {
                  const decoded = this.api.registry.findMetaError(dispatchError.asModule);
                  reject(new TransactionError(`${decoded.section}.${decoded.name}: ${decoded.docs.join(' ')}`));
                } else {
                  reject(new TransactionError(dispatchError.toString()));
                }
              } else {
                resolve(status.asFinalized.toHex());
              }
            }
          });
      });
    } catch (error) {
      throw new TransactionError(`Failed to update profile: ${error.message}`);
    }
  }

  /**
   * Gets an AI profile
   *
   * @param did - AI DID
   * @returns Promise resolving to AI profile or null if not found
   *
   * @example
   * ```typescript
   * const profile = await aidid.getProfile(aiDid);
   * console.log('AI Type:', profile.aiType);
   * console.log('Version:', profile.version);
   * console.log('Capabilities:', profile.capabilities.tasks);
   * ```
   */
  async getProfile(did: AIDID): Promise<AIProfile | null> {
    try {
      const profileOption = await this.api.query.aidid.aiProfiles(did);

      if (profileOption.isNone) {
        return null;
      }

      const profile = profileOption.unwrap();

      return {
        aiType: this.parseAIType(profile.aiType),
        version: profile.version.toString(),
        architecture: profile.architecture.toString(),
        parameters: profile.parameters.toString(),
        capabilities: this.decodeCapabilities(profile.capabilities),
        restrictions: this.decodeRestrictions(profile.restrictions),
        safety: this.decodeSafety(profile.safety),
      };
    } catch (error) {
      throw new TransactionError(`Failed to get profile: ${error.message}`);
    }
  }

  /**
   * Gets AI reputation score
   *
   * @param did - AI DID
   * @returns Promise resolving to reputation details
   *
   * @example
   * ```typescript
   * const reputation = await aidid.getReputation(aiDid);
   * console.log('Score:', reputation.score / 100);  // Convert to percentage
   * console.log('Success Rate:', reputation.successRate / 100);
   * console.log('Total Inferences:', reputation.totalInferences);
   * ```
   */
  async getReputation(did: AIDID): Promise<Reputation> {
    try {
      const reputationOption = await this.api.query.aidid.reputations(did);

      if (reputationOption.isNone) {
        // Default reputation for new AI
        return {
          score: 10000,
          totalInferences: 0,
          successfulInferences: 0,
          failedInferences: 0,
          userRating: 0,
          ratingCount: 0,
          uptime: 10000,
          incidents: 0,
          successRate: 10000,
        };
      }

      const rep = reputationOption.unwrap();
      const totalInferences = rep.totalInferences.toNumber();
      const successfulInferences = rep.successfulInferences.toNumber();
      const successRate = totalInferences > 0
        ? Math.floor((successfulInferences * 10000) / totalInferences)
        : 10000;

      return {
        score: rep.score.toNumber(),
        totalInferences,
        successfulInferences,
        failedInferences: rep.failedInferences.toNumber(),
        userRating: rep.userRating.toNumber(),
        ratingCount: rep.ratingCount.toNumber(),
        uptime: rep.uptime.toNumber(),
        incidents: rep.incidents.toNumber(),
        successRate,
      };
    } catch (error) {
      throw new TransactionError(`Failed to get reputation: ${error.message}`);
    }
  }

  /**
   * Records an inference result
   *
   * Logs an AI inference to update reputation score.
   *
   * @param signer - AI operator account
   * @param did - AI DID
   * @param success - Whether inference succeeded
   * @param metadata - Optional inference metadata
   * @returns Promise resolving to transaction hash
   *
   * @example
   * ```typescript
   * const metadata: InferenceMetadata = {
   *   task: Task.TextGeneration,
   *   inputSize: 100,
   *   outputSize: 500,
   *   processingTime: 1200,  // ms
   * };
   *
   * await aidid.recordInference(alice, aiDid, true, metadata);
   * ```
   */
  async recordInference(
    signer: KeyringPair,
    did: AIDID,
    success: boolean,
    metadata?: InferenceMetadata
  ): Promise<string> {
    try {
      return new Promise((resolve, reject) => {
        this.api.tx.aidid
          .recordInference(
            did,
            success,
            metadata ? this.encodeInferenceMetadata(metadata) : null
          )
          .signAndSend(signer, ({ status, dispatchError }) => {
            if (status.isFinalized) {
              if (dispatchError) {
                if (dispatchError.isModule) {
                  const decoded = this.api.registry.findMetaError(dispatchError.asModule);
                  reject(new TransactionError(`${decoded.section}.${decoded.name}: ${decoded.docs.join(' ')}`));
                } else {
                  reject(new TransactionError(dispatchError.toString()));
                }
              } else {
                resolve(status.asFinalized.toHex());
              }
            }
          });
      });
    } catch (error) {
      throw new TransactionError(`Failed to record inference: ${error.message}`);
    }
  }

  /**
   * Adds a user rating for an AI
   *
   * @param signer - User account
   * @param did - AI DID
   * @param rating - Rating (0-10000 representing 0.00 - 100.00)
   * @param review - Optional text review
   * @returns Promise resolving to transaction hash
   *
   * @example
   * ```typescript
   * await aidid.addRating(
   *   alice,
   *   aiDid,
   *   8500,  // 85.00%
   *   'Excellent performance on text generation tasks'
   * );
   * ```
   */
  async addRating(
    signer: KeyringPair,
    did: AIDID,
    rating: number,
    review?: string
  ): Promise<string> {
    if (rating < 0 || rating > 10000) {
      throw new ValidationError('Rating must be between 0 and 10000');
    }

    try {
      return new Promise((resolve, reject) => {
        this.api.tx.aidid
          .addRating(did, rating, review || '')
          .signAndSend(signer, ({ status, dispatchError }) => {
            if (status.isFinalized) {
              if (dispatchError) {
                if (dispatchError.isModule) {
                  const decoded = this.api.registry.findMetaError(dispatchError.asModule);
                  reject(new TransactionError(`${decoded.section}.${decoded.name}: ${decoded.docs.join(' ')}`));
                } else {
                  reject(new TransactionError(dispatchError.toString()));
                }
              } else {
                resolve(status.asFinalized.toHex());
              }
            }
          });
      });
    } catch (error) {
      throw new TransactionError(`Failed to add rating: ${error.message}`);
    }
  }

  /**
   * Grants a permission to an AI
   *
   * @param signer - Permission granter account
   * @param did - AI DID
   * @param action - Action to permit
   * @param resource - Resource action applies to
   * @param conditions - Optional conditions
   * @returns Promise resolving to permission ID
   *
   * @example
   * ```typescript
   * const permissionId = await aidid.grantPermission(
   *   alice,
   *   aiDid,
   *   'read',
   *   'user_data',
   *   ['only_public_data', 'audit_trail_required']
   * );
   * ```
   */
  async grantPermission(
    signer: KeyringPair,
    did: AIDID,
    action: string,
    resource: string,
    conditions: string[] = []
  ): Promise<string> {
    try {
      return new Promise((resolve, reject) => {
        this.api.tx.aidid
          .grantPermission(did, action, resource, conditions)
          .signAndSend(signer, ({ status, events, dispatchError }) => {
            if (status.isFinalized) {
              if (dispatchError) {
                if (dispatchError.isModule) {
                  const decoded = this.api.registry.findMetaError(dispatchError.asModule);
                  reject(new TransactionError(`${decoded.section}.${decoded.name}: ${decoded.docs.join(' ')}`));
                } else {
                  reject(new TransactionError(dispatchError.toString()));
                }
              } else {
                // Find PermissionGranted event
                const permEvent = events.find(({ event }) =>
                  this.api.events.aidid.PermissionGranted.is(event)
                );

                if (permEvent) {
                  const [permissionId] = permEvent.event.data;
                  resolve(permissionId.toString());
                } else {
                  reject(new TransactionError('PermissionGranted event not found'));
                }
              }
            }
          });
      });
    } catch (error) {
      throw new TransactionError(`Failed to grant permission: ${error.message}`);
    }
  }

  /**
   * Revokes a permission from an AI
   *
   * @param signer - Permission granter account
   * @param did - AI DID
   * @param permissionId - Permission ID to revoke
   * @returns Promise resolving to transaction hash
   *
   * @example
   * ```typescript
   * await aidid.revokePermission(alice, aiDid, permissionId);
   * ```
   */
  async revokePermission(
    signer: KeyringPair,
    did: AIDID,
    permissionId: string
  ): Promise<string> {
    try {
      return new Promise((resolve, reject) => {
        this.api.tx.aidid
          .revokePermission(did, permissionId)
          .signAndSend(signer, ({ status, dispatchError }) => {
            if (status.isFinalized) {
              if (dispatchError) {
                if (dispatchError.isModule) {
                  const decoded = this.api.registry.findMetaError(dispatchError.asModule);
                  reject(new TransactionError(`${decoded.section}.${decoded.name}: ${decoded.docs.join(' ')}`));
                } else {
                  reject(new TransactionError(dispatchError.toString()));
                }
              } else {
                resolve(status.asFinalized.toHex());
              }
            }
          });
      });
    } catch (error) {
      throw new TransactionError(`Failed to revoke permission: ${error.message}`);
    }
  }

  /**
   * Queries AIs by capability
   *
   * Searches for AIs that can perform a specific task.
   *
   * @param task - Task to search for
   * @param minReputation - Minimum reputation score (0-10000)
   * @returns Promise resolving to array of matching AIs
   *
   * @example
   * ```typescript
   * const ais = await aidid.queryByCapability(
   *   Task.TextGeneration,
   *   7500  // Min 75% reputation
   * );
   *
   * ais.forEach(ai => {
   *   console.log(`${ai.did}: ${ai.reputation.score / 100}% score`);
   * });
   * ```
   */
  async queryByCapability(
    task: Task,
    minReputation: number = 0
  ): Promise<AISearchResult[]> {
    try {
      // Get all registered AIs
      const entries = await this.api.query.aidid.aiProfiles.entries();
      const results: AISearchResult[] = [];

      for (const [key, profileOption] of entries) {
        if (profileOption.isNone) continue;

        const did = key.args[0].toString();
        const profile = await this.getProfile(did);
        const reputation = await this.getReputation(did);

        if (!profile) continue;

        // Check if AI can perform this task
        const canPerformTask = profile.capabilities.tasks.includes(task);
        const meetsReputation = reputation.score >= minReputation;

        if (canPerformTask && meetsReputation) {
          // Calculate match score (0-100)
          const matchScore = this.calculateMatchScore(task, profile, reputation);

          results.push({
            did,
            profile,
            reputation,
            matchScore,
          });
        }
      }

      // Sort by match score (highest first)
      results.sort((a, b) => b.matchScore - a.matchScore);

      return results;
    } catch (error) {
      throw new TransactionError(`Failed to query by capability: ${error.message}`);
    }
  }

  /**
   * Gets all permissions for an AI
   *
   * @param did - AI DID
   * @returns Promise resolving to array of permissions
   */
  async getPermissions(did: AIDID): Promise<Permission[]> {
    try {
      const permissionsOption = await this.api.query.aidid.permissions(did);

      if (permissionsOption.isNone) {
        return [];
      }

      const permissions = permissionsOption.unwrap();
      return permissions.map((perm: any) => ({
        id: perm.id.toString(),
        action: perm.action.toString(),
        resource: perm.resource.toString(),
        conditions: perm.conditions.map((c: any) => c.toString()),
        grantedAt: perm.grantedAt.toNumber(),
        grantedBy: perm.grantedBy.toString(),
      }));
    } catch (error) {
      throw new TransactionError(`Failed to get permissions: ${error.message}`);
    }
  }

  // Private helper methods

  private validateProfile(profile: AIProfile): void {
    if (!profile.version || profile.version.trim() === '') {
      throw new ValidationError('Version is required');
    }
    if (!profile.architecture || profile.architecture.trim() === '') {
      throw new ValidationError('Architecture is required');
    }
    if (profile.capabilities.tasks.length === 0) {
      throw new ValidationError('At least one task capability is required');
    }
  }

  private encodeCapabilities(cap: Capabilities): any {
    return {
      tasks: cap.tasks,
      inputModalities: cap.inputModalities,
      outputModalities: cap.outputModalities,
      languages: cap.languages,
      maxContext: cap.maxContext || null,
      maxOutput: cap.maxOutput || null,
    };
  }

  private decodeCapabilities(cap: any): Capabilities {
    return {
      tasks: cap.tasks.toJSON(),
      inputModalities: cap.inputModalities.toJSON(),
      outputModalities: cap.outputModalities.toJSON(),
      languages: cap.languages.toJSON(),
      maxContext: cap.maxContext.isSome ? cap.maxContext.unwrap().toNumber() : undefined,
      maxOutput: cap.maxOutput.isSome ? cap.maxOutput.unwrap().toNumber() : undefined,
    };
  }

  private encodeRestrictions(rest: Restrictions): any {
    return {
      prohibitedTasks: rest.prohibitedTasks,
      contentFiltering: rest.contentFiltering,
      requiresSupervision: rest.requiresSupervision,
      noRealtimeData: rest.noRealtimeData,
      knowledgeCutoff: rest.knowledgeCutoff || null,
      rateLimit: rest.rateLimit || null,
    };
  }

  private decodeRestrictions(rest: any): Restrictions {
    return {
      prohibitedTasks: rest.prohibitedTasks.toJSON(),
      contentFiltering: rest.contentFiltering.isTrue,
      requiresSupervision: rest.requiresSupervision.isTrue,
      noRealtimeData: rest.noRealtimeData.isTrue,
      knowledgeCutoff: rest.knowledgeCutoff.isSome ? rest.knowledgeCutoff.unwrap().toNumber() : undefined,
      rateLimit: rest.rateLimit.isSome ? rest.rateLimit.unwrap().toNumber() : undefined,
    };
  }

  private encodeSafety(safety: SafetyProfile): any {
    return {
      alignmentMethod: safety.alignmentMethod,
      contentFiltering: safety.contentFiltering,
      biasEvaluated: safety.biasEvaluated,
      toxicityScore: safety.toxicityScore,
    };
  }

  private decodeSafety(safety: any): SafetyProfile {
    return {
      alignmentMethod: safety.alignmentMethod.toString(),
      contentFiltering: safety.contentFiltering.isTrue,
      biasEvaluated: safety.biasEvaluated.isTrue,
      toxicityScore: safety.toxicityScore.toNumber(),
    };
  }

  private encodeInferenceMetadata(metadata: InferenceMetadata): any {
    return {
      task: metadata.task,
      inputSize: metadata.inputSize,
      outputSize: metadata.outputSize,
      processingTime: metadata.processingTime,
      cost: metadata.cost ? metadata.cost.toString() : null,
    };
  }

  private parseAIType(type: any): AIType {
    const typeStr = type.toString();
    return AIType[typeStr as keyof typeof AIType] || AIType.Agent;
  }

  private calculateMatchScore(task: Task, profile: AIProfile, reputation: Reputation): number {
    // Base score from reputation (0-50 points)
    const reputationScore = (reputation.score / 10000) * 50;

    // Task specificity bonus (0-30 points)
    const taskCount = profile.capabilities.tasks.length;
    const specificityBonus = Math.max(0, 30 - (taskCount * 2));

    // Success rate bonus (0-20 points)
    const successBonus = (reputation.successRate / 10000) * 20;

    const total = reputationScore + specificityBonus + successBonus;
    return Math.min(100, Math.round(total));
  }
}
