# Beta Feedback Collection System

Comprehensive feedback collection across all platforms with automated triage and response.

## Feedback Channels

### 1. In-App Feedback (Primary)

#### iOS Implementation
```swift
// ios/EtridWallet/FeedbackView.swift
import SwiftUI

struct FeedbackView: View {
    @State private var feedbackType: FeedbackType = .bug
    @State private var feedback: String = ""
    @State private var screenshot: UIImage?
    @State private var isSubmitting = false
    @State private var submitted = false

    enum FeedbackType: String, CaseIterable {
        case bug = "Bug Report"
        case feature = "Feature Request"
        case general = "General Feedback"
    }

    var body: some View {
        NavigationView {
            Form {
                Section(header: Text("Feedback Type")) {
                    Picker("Type", selection: $feedbackType) {
                        ForEach(FeedbackType.allCases, id: \.self) { type in
                            Text(type.rawValue).tag(type)
                        }
                    }
                    .pickerStyle(SegmentedPickerStyle())
                }

                Section(header: Text("Details")) {
                    TextEditor(text: $feedback)
                        .frame(minHeight: 150)
                }

                Section(header: Text("Screenshot (Optional)")) {
                    if let screenshot = screenshot {
                        Image(uiImage: screenshot)
                            .resizable()
                            .scaledToFit()
                            .frame(maxHeight: 200)
                    }

                    Button("Capture Screenshot") {
                        captureScreenshot()
                    }
                }

                Section {
                    Button(action: submitFeedback) {
                        if isSubmitting {
                            ProgressView()
                        } else {
                            Text("Submit Feedback")
                        }
                    }
                    .disabled(feedback.isEmpty || isSubmitting)
                }
            }
            .navigationTitle("Send Feedback")
            .alert("Thank You!", isPresented: $submitted) {
                Button("OK") {
                    // Dismiss view
                }
            } message: {
                Text("Your feedback has been submitted.")
            }
        }
    }

    func captureScreenshot() {
        let window = UIApplication.shared.windows.first!
        let renderer = UIGraphicsImageRenderer(bounds: window.bounds)
        screenshot = renderer.image { context in
            window.drawHierarchy(in: window.bounds, afterScreenUpdates: true)
        }
    }

    func submitFeedback() {
        isSubmitting = true

        let feedbackData: [String: Any] = [
            "type": feedbackType.rawValue,
            "feedback": feedback,
            "platform": "ios",
            "version": Bundle.main.infoDictionary?["CFBundleShortVersionString"] as? String ?? "",
            "device": UIDevice.current.model,
            "os_version": UIDevice.current.systemVersion,
            "timestamp": ISO8601DateFormatter().string(from: Date())
        ]

        // Upload screenshot if exists
        var screenshotUrl: String?
        if let screenshot = screenshot {
            screenshotUrl = uploadScreenshot(screenshot)
        }

        // Submit to API
        submitToAPI(feedbackData, screenshotUrl: screenshotUrl) { success in
            isSubmitting = false
            if success {
                submitted = true
            }
        }
    }
}
```

#### Android Implementation
```kotlin
// android/app/src/main/java/com/etrid/wallet/FeedbackActivity.kt
package com.etrid.wallet

import android.graphics.Bitmap
import android.os.Bundle
import android.view.View
import androidx.appcompat.app.AppCompatActivity
import kotlinx.android.synthetic.main.activity_feedback.*
import kotlinx.coroutines.launch

class FeedbackActivity : AppCompatActivity() {
    private var screenshot: Bitmap? = null

    enum class FeedbackType {
        BUG, FEATURE, GENERAL
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_feedback)

        setupUI()
    }

    private fun setupUI() {
        // Type selector
        radioGroupType.setOnCheckedChangeListener { _, checkedId ->
            // Handle type selection
        }

        // Screenshot button
        btnScreenshot.setOnClickListener {
            captureScreenshot()
        }

        // Submit button
        btnSubmit.setOnClickListener {
            submitFeedback()
        }
    }

    private fun captureScreenshot() {
        val rootView = window.decorView.rootView
        rootView.isDrawingCacheEnabled = true
        screenshot = Bitmap.createBitmap(rootView.drawingCache)
        rootView.isDrawingCacheEnabled = false

        imgScreenshot.setImageBitmap(screenshot)
        imgScreenshot.visibility = View.VISIBLE
    }

    private fun submitFeedback() {
        val type = when (radioGroupType.checkedRadioButtonId) {
            R.id.radioBug -> FeedbackType.BUG
            R.id.radioFeature -> FeedbackType.FEATURE
            else -> FeedbackType.GENERAL
        }

        val feedback = etFeedback.text.toString()

        if (feedback.isBlank()) {
            toast("Please enter your feedback")
            return
        }

        progressBar.visibility = View.VISIBLE
        btnSubmit.isEnabled = false

        lifecycleScope.launch {
            try {
                val data = mapOf(
                    "type" to type.name,
                    "feedback" to feedback,
                    "platform" to "android",
                    "version" to BuildConfig.VERSION_NAME,
                    "device" to "${Build.MANUFACTURER} ${Build.MODEL}",
                    "os_version" to Build.VERSION.RELEASE,
                    "timestamp" to System.currentTimeMillis()
                )

                // Upload screenshot if exists
                val screenshotUrl = screenshot?.let { uploadScreenshot(it) }

                // Submit to API
                val success = api.submitFeedback(data, screenshotUrl)

                if (success) {
                    toast("Thank you for your feedback!")
                    finish()
                } else {
                    toast("Failed to submit. Please try again.")
                }
            } catch (e: Exception) {
                toast("Error: ${e.message}")
            } finally {
                progressBar.visibility = View.GONE
                btnSubmit.isEnabled = true
            }
        }
    }
}
```

#### React Native Implementation
```typescript
// components/FeedbackModal.tsx
import React, { useState } from 'react';
import { View, Text, TextInput, TouchableOpacity, Image, Alert } from 'react-native';
import { captureRef } from 'react-native-view-shot';
import { submitFeedback } from '@/lib/api/feedback';

export function FeedbackModal({ onClose }: { onClose: () => void }) {
  const [type, setType] = useState<'bug' | 'feature' | 'general'>('bug');
  const [feedback, setFeedback] = useState('');
  const [screenshot, setScreenshot] = useState<string | null>(null);
  const [submitting, setSubmitting] = useState(false);

  async function handleScreenshot() {
    try {
      const uri = await captureRef(viewRef, {
        format: 'png',
        quality: 0.8,
      });
      setScreenshot(uri);
    } catch (error) {
      Alert.alert('Error', 'Failed to capture screenshot');
    }
  }

  async function handleSubmit() {
    if (!feedback.trim()) {
      Alert.alert('Error', 'Please enter your feedback');
      return;
    }

    setSubmitting(true);

    try {
      await submitFeedback({
        type,
        feedback,
        screenshot,
        platform: Platform.OS,
        version: DeviceInfo.getVersion(),
        device: `${DeviceInfo.getBrand()} ${DeviceInfo.getModel()}`,
        osVersion: DeviceInfo.getSystemVersion(),
      });

      Alert.alert('Thank You!', 'Your feedback has been submitted.');
      onClose();
    } catch (error) {
      Alert.alert('Error', 'Failed to submit feedback. Please try again.');
    } finally {
      setSubmitting(false);
    }
  }

  return (
    <View style={styles.container}>
      <Text style={styles.title}>Send Feedback</Text>

      {/* Type selector */}
      <View style={styles.typeSelector}>
        <TouchableOpacity
          style={[styles.typeButton, type === 'bug' && styles.typeButtonActive]}
          onPress={() => setType('bug')}
        >
          <Text>🐛 Bug</Text>
        </TouchableOpacity>
        <TouchableOpacity
          style={[styles.typeButton, type === 'feature' && styles.typeButtonActive]}
          onPress={() => setType('feature')}
        >
          <Text>💡 Feature</Text>
        </TouchableOpacity>
        <TouchableOpacity
          style={[styles.typeButton, type === 'general' && styles.typeButtonActive]}
          onPress={() => setType('general')}
        >
          <Text>💬 General</Text>
        </TouchableOpacity>
      </View>

      {/* Feedback input */}
      <TextInput
        style={styles.input}
        placeholder="Describe your feedback..."
        multiline
        numberOfLines={6}
        value={feedback}
        onChangeText={setFeedback}
      />

      {/* Screenshot */}
      {screenshot && (
        <Image source={{ uri: screenshot }} style={styles.screenshot} />
      )}

      <TouchableOpacity style={styles.screenshotButton} onPress={handleScreenshot}>
        <Text>📷 Add Screenshot</Text>
      </TouchableOpacity>

      {/* Submit button */}
      <TouchableOpacity
        style={[styles.submitButton, submitting && styles.submitButtonDisabled]}
        onPress={handleSubmit}
        disabled={submitting}
      >
        <Text style={styles.submitButtonText}>
          {submitting ? 'Submitting...' : 'Submit Feedback'}
        </Text>
      </TouchableOpacity>

      <TouchableOpacity style={styles.cancelButton} onPress={onClose}>
        <Text>Cancel</Text>
      </TouchableOpacity>
    </View>
  );
}
```

### 2. Email Feedback

**beta@etrid.com** → Auto-categorized and tracked

#### Email Parser Configuration
```typescript
// lib/email-parser.ts
import { parseEmail } from '@/lib/utils';

export async function processIncomingEmail(email: any) {
  const parsed = {
    from: email.from,
    subject: email.subject,
    body: email.body,
    attachments: email.attachments,
    receivedAt: new Date(),
  };

  // Auto-categorize based on subject
  let type: 'bug' | 'feature' | 'general' = 'general';
  const subject = email.subject.toLowerCase();

  if (subject.includes('bug') || subject.includes('error') || subject.includes('crash')) {
    type = 'bug';
  } else if (subject.includes('feature') || subject.includes('request') || subject.includes('suggestion')) {
    type = 'feature';
  }

  // Extract device info from signature if present
  const deviceInfo = extractDeviceInfo(email.body);

  // Create feedback entry
  await createFeedback({
    type,
    source: 'email',
    content: email.body,
    email: email.from,
    deviceInfo,
    attachments: email.attachments,
  });

  // Send auto-reply
  await sendAutoReply(email.from, type);
}

async function sendAutoReply(to: string, type: string) {
  const template = {
    bug: `Thank you for reporting this bug! We've received your report and our team will investigate. You'll hear from us within 24 hours.`,
    feature: `Thanks for the feature request! We'll review it and add it to our roadmap if it aligns with our vision.`,
    general: `Thank you for your feedback! We appreciate you taking the time to share your thoughts.`,
  };

  // Send email via SendGrid, Resend, etc.
}
```

### 3. Discord Integration

**#beta-testing channel** → Synced to feedback database

#### Discord Bot Configuration
```typescript
// bots/discord-feedback.ts
import { Client, GatewayIntentBits } from 'discord.js';

const client = new Client({
  intents: [
    GatewayIntentBits.Guilds,
    GatewayIntentBits.GuildMessages,
    GatewayIntentBits.MessageContent,
  ],
});

client.on('messageCreate', async (message) => {
  // Ignore bot messages
  if (message.author.bot) return;

  // Only process messages in #beta-testing channel
  if (message.channel.id !== process.env.DISCORD_BETA_CHANNEL_ID) return;

  // Categorize with reactions
  await message.react('🐛'); // Bug
  await message.react('💡'); // Feature
  await message.react('💬'); // General

  // Track feedback
  await createFeedback({
    type: 'general', // Updated based on reaction
    source: 'discord',
    content: message.content,
    author: message.author.tag,
    discordId: message.author.id,
    messageUrl: message.url,
    attachments: message.attachments.map((a) => a.url),
  });

  // Reply
  await message.reply(
    'Thanks for the feedback! React with 🐛 for bug, 💡 for feature request, or 💬 for general feedback.'
  );
});

// Track reactions for categorization
client.on('messageReactionAdd', async (reaction, user) => {
  if (user.bot) return;

  const emoji = reaction.emoji.name;
  const type =
    emoji === '🐛' ? 'bug' : emoji === '💡' ? 'feature' : 'general';

  // Update feedback type
  await updateFeedbackType(reaction.message.id, type);
});

client.login(process.env.DISCORD_BOT_TOKEN);
```

### 4. Twitter/X Monitoring

**@EtridWallet mentions** → Tracked and responded to

```typescript
// bots/twitter-monitor.ts
import { TwitterApi } from 'twitter-api-v2';

const client = new TwitterApi(process.env.TWITTER_BEARER_TOKEN!);

async function monitorMentions() {
  const stream = await client.v2.searchStream({
    'tweet.fields': ['author_id', 'created_at', 'conversation_id'],
    expansions: ['author_id'],
  });

  // Add rules to monitor
  await client.v2.updateStreamRules({
    add: [
      { value: '@EtridWallet beta', tag: 'beta_feedback' },
      { value: '@EtridWallet bug', tag: 'bug_report' },
      { value: '@EtridWallet feature', tag: 'feature_request' },
    ],
  });

  // Process tweets
  stream.on('data', async (tweet) => {
    await createFeedback({
      type: getTweetType(tweet),
      source: 'twitter',
      content: tweet.data.text,
      twitterId: tweet.data.id,
      authorId: tweet.data.author_id,
      url: `https://twitter.com/i/web/status/${tweet.data.id}`,
    });

    // Auto-reply
    await replyToTweet(tweet.data.id, 'Thanks for the feedback! Our team will review this.');
  });
}

monitorMentions();
```

### 5. TestFlight Feedback (iOS)

**Automatically imported from App Store Connect**

```typescript
// scripts/import-testflight-feedback.ts
import { AppStoreConnectAPI } from 'app-store-connect-api';

async function importTestFlightFeedback() {
  const api = new AppStoreConnectAPI({
    issuerId: process.env.ASC_ISSUER_ID!,
    keyId: process.env.ASC_KEY_ID!,
    privateKey: process.env.ASC_PRIVATE_KEY!,
  });

  // Get beta feedback
  const feedback = await api.betaFeedback.listBetaFeedback({
    filter: {
      betaAppId: process.env.ASC_BETA_APP_ID!,
    },
  });

  for (const item of feedback.data) {
    await createFeedback({
      type: 'general',
      source: 'testflight',
      content: item.attributes.comment,
      email: item.attributes.email,
      deviceInfo: {
        model: item.attributes.deviceType,
        os: item.attributes.osVersion,
      },
      buildVersion: item.attributes.buildVersion,
      screenshot: item.attributes.screenshot,
    });
  }
}

// Run hourly
setInterval(importTestFlightFeedback, 60 * 60 * 1000);
```

### 6. Google Play Reviews (Android)

**Automatically imported from Play Console**

```typescript
// scripts/import-play-reviews.ts
import { google } from 'googleapis';

async function importPlayReviews() {
  const auth = new google.auth.GoogleAuth({
    keyFile: 'google-play-service-account.json',
    scopes: ['https://www.googleapis.com/auth/androidpublisher'],
  });

  const androidpublisher = google.androidpublisher({
    version: 'v3',
    auth: await auth.getClient(),
  });

  const response = await androidpublisher.reviews.list({
    packageName: 'com.etrid.wallet',
  });

  for (const review of response.data.reviews || []) {
    // Only process beta reviews
    if (!review.comments?.[0]?.userComment?.reviewerLanguage) continue;

    await createFeedback({
      type: review.comments[0].userComment.starRating < 3 ? 'bug' : 'general',
      source: 'play_store',
      content: review.comments[0].userComment.text,
      rating: review.comments[0].userComment.starRating,
      deviceInfo: {
        model: review.comments[0].userComment.deviceMetadata?.productName,
        os: review.comments[0].userComment.androidOsVersion,
      },
      reviewId: review.reviewId,
    });

    // Reply to review
    if (shouldReplyToReview(review)) {
      await androidpublisher.reviews.reply({
        packageName: 'com.etrid.wallet',
        reviewId: review.reviewId!,
        requestBody: {
          replyText: generateReplyText(review),
        },
      });
    }
  }
}

// Run every 6 hours
setInterval(importPlayReviews, 6 * 60 * 60 * 1000);
```

## Feedback API

**`app/api/feedback/route.ts`:**
```typescript
import { NextRequest, NextResponse } from 'next/server';
import { z } from 'zod';
import { prisma } from '@/lib/prisma';
import { uploadToS3 } from '@/lib/s3';

const feedbackSchema = z.object({
  type: z.enum(['bug', 'feature', 'general']),
  feedback: z.string().min(10),
  screenshot: z.string().optional(),
  url: z.string().optional(),
  userAgent: z.string().optional(),
  platform: z.enum(['ios', 'android', 'pwa']),
  version: z.string(),
  device: z.string().optional(),
  osVersion: z.string().optional(),
});

export async function POST(request: NextRequest) {
  try {
    const body = await request.json();
    const data = feedbackSchema.parse(body);

    // Upload screenshot if provided
    let screenshotUrl: string | undefined;
    if (data.screenshot) {
      screenshotUrl = await uploadToS3(data.screenshot, 'feedback-screenshots');
    }

    // Create feedback entry
    const feedback = await prisma.feedback.create({
      data: {
        type: data.type,
        content: data.feedback,
        source: 'in_app',
        platform: data.platform,
        version: data.version,
        device: data.device,
        osVersion: data.osVersion,
        url: data.url,
        userAgent: data.userAgent,
        screenshot: screenshotUrl,
        userId: request.headers.get('user-id'),
      },
    });

    // Auto-triage
    await triageFeedback(feedback);

    // Notify team if critical
    if (feedback.severity === 'critical') {
      await notifyTeam(feedback);
    }

    return NextResponse.json({
      success: true,
      feedbackId: feedback.id,
    });
  } catch (error) {
    console.error('Feedback submission error:', error);
    return NextResponse.json(
      { error: 'Failed to submit feedback' },
      { status: 500 }
    );
  }
}

async function triageFeedback(feedback: any) {
  // AI-powered categorization and severity detection
  const keywords = {
    critical: ['crash', 'data loss', 'funds', 'security', 'hack'],
    high: ['broken', 'not working', 'error', 'failed'],
    medium: ['slow', 'confusing', 'unclear'],
    low: ['polish', 'minor', 'suggestion'],
  };

  const content = feedback.content.toLowerCase();
  let severity = 'low';

  for (const [level, words] of Object.entries(keywords)) {
    if (words.some((word) => content.includes(word))) {
      severity = level;
      break;
    }
  }

  // Update severity
  await prisma.feedback.update({
    where: { id: feedback.id },
    data: { severity },
  });
}

async function notifyTeam(feedback: any) {
  // Send Slack notification
  await fetch(process.env.SLACK_WEBHOOK_URL!, {
    method: 'POST',
    body: JSON.stringify({
      text: `🚨 Critical Feedback Received`,
      blocks: [
        {
          type: 'section',
          text: {
            type: 'mrkdwn',
            text: `*Critical ${feedback.type} reported*\n${feedback.content.substring(0, 200)}...`,
          },
        },
        {
          type: 'section',
          fields: [
            { type: 'mrkdwn', text: `*Platform:*\n${feedback.platform}` },
            { type: 'mrkdwn', text: `*Version:*\n${feedback.version}` },
          ],
        },
        {
          type: 'actions',
          elements: [
            {
              type: 'button',
              text: { type: 'plain_text', text: 'View Feedback' },
              url: `https://wallet.etrid.com/dashboard/feedback/${feedback.id}`,
            },
          ],
        },
      ],
    }),
  });
}
```

## Automated Triage

**`lib/feedback-triage.ts`:**
```typescript
/**
 * Automatically categorize and prioritize feedback
 */

export async function automaticTriage(feedbackId: string) {
  const feedback = await prisma.feedback.findUnique({
    where: { id: feedbackId },
  });

  if (!feedback) return;

  const analysis = {
    severity: detectSeverity(feedback.content),
    category: detectCategory(feedback.content),
    tags: extractTags(feedback.content),
    sentiment: analyzeSentiment(feedback.content),
    duplicateOf: await findDuplicates(feedback),
  };

  // Update feedback with analysis
  await prisma.feedback.update({
    where: { id: feedbackId },
    data: {
      severity: analysis.severity,
      category: analysis.category,
      tags: analysis.tags,
      sentiment: analysis.sentiment,
      duplicateOf: analysis.duplicateOf,
      triaged: true,
    },
  });

  // Create Jira/Linear ticket for bugs
  if (feedback.type === 'bug' && analysis.severity !== 'low') {
    await createTicket(feedback, analysis);
  }

  // Add to product roadmap for features
  if (feedback.type === 'feature' && analysis.sentiment > 0.7) {
    await addToRoadmap(feedback);
  }

  return analysis;
}

function detectSeverity(content: string): string {
  const critical = ['crash', 'data loss', 'security', 'funds', 'hack', 'stolen'];
  const high = ['broken', 'not working', 'error', 'failed', 'unusable'];
  const medium = ['slow', 'confusing', 'difficult', 'unclear'];

  const lower = content.toLowerCase();

  if (critical.some((word) => lower.includes(word))) return 'critical';
  if (high.some((word) => lower.includes(word))) return 'high';
  if (medium.some((word) => lower.includes(word))) return 'medium';
  return 'low';
}

function detectCategory(content: string): string {
  const categories = {
    wallet: ['wallet', 'balance', 'transaction', 'send', 'receive'],
    bloccard: ['card', 'bloccard', 'payment', 'purchase', 'spend'],
    trading: ['trade', 'swap', 'exchange', 'price', 'slippage'],
    nft: ['nft', 'collectible', 'gallery', 'mint'],
    security: ['security', 'biometric', 'password', 'recovery', 'backup'],
    ui: ['ui', 'design', 'layout', 'button', 'screen'],
    performance: ['slow', 'lag', 'freeze', 'loading', 'crash'],
  };

  const lower = content.toLowerCase();

  for (const [category, keywords] of Object.entries(categories)) {
    if (keywords.some((word) => lower.includes(word))) {
      return category;
    }
  }

  return 'other';
}

function extractTags(content: string): string[] {
  const tags: string[] = [];
  const lower = content.toLowerCase();

  const tagMap = {
    'crash': 'crash',
    'bug': 'bug',
    'feature': 'feature-request',
    'slow': 'performance',
    'confusing': 'ux',
    'error': 'error',
    'security': 'security',
    'ios': 'ios',
    'android': 'android',
    'web': 'pwa',
  };

  for (const [keyword, tag] of Object.entries(tagMap)) {
    if (lower.includes(keyword)) {
      tags.push(tag);
    }
  }

  return [...new Set(tags)]; // Remove duplicates
}

async function findDuplicates(feedback: any): Promise<string | null> {
  // Use vector similarity or keyword matching to find duplicates
  const similar = await prisma.feedback.findMany({
    where: {
      type: feedback.type,
      platform: feedback.platform,
      category: feedback.category,
      createdAt: {
        gte: new Date(Date.now() - 7 * 24 * 60 * 60 * 1000), // Last 7 days
      },
    },
  });

  for (const item of similar) {
    if (calculateSimilarity(feedback.content, item.content) > 0.8) {
      return item.id;
    }
  }

  return null;
}

function calculateSimilarity(text1: string, text2: string): number {
  // Simple Jaccard similarity
  const words1 = new Set(text1.toLowerCase().split(/\s+/));
  const words2 = new Set(text2.toLowerCase().split(/\s+/));

  const intersection = new Set([...words1].filter((x) => words2.has(x)));
  const union = new Set([...words1, ...words2]);

  return intersection.size / union.size;
}

async function createTicket(feedback: any, analysis: any) {
  // Create Linear/Jira ticket
  // Implementation depends on your project management tool
}
```

## Response Templates

**`lib/feedback-responses.ts`:**
```typescript
export const responseTemplates = {
  bug: {
    acknowledged: `Thank you for reporting this bug! We've received your report and our engineering team will investigate. We'll keep you updated on progress.`,

    investigating: `We're investigating this issue. Our team is working to reproduce it and identify the root cause. Expected resolution: 2-3 business days.`,

    fixed: `Good news! This bug has been fixed in version {{version}}. Please update your app and let us know if you're still experiencing issues.`,

    duplicate: `Thanks for reporting! This appears to be a duplicate of issue #{{issueId}}, which we're already working on. We've added your report to increase priority.`,
  },

  feature: {
    received: `Thanks for the feature request! We've added it to our roadmap for consideration. We'll update you if it's prioritized.`,

    planned: `Great news! This feature is now on our roadmap. We're planning to implement it in Q{{quarter}}. Stay tuned!`,

    implemented: `Your feature request has been implemented! Check it out in version {{version}}.`,

    declined: `Thanks for the suggestion. After careful consideration, we've decided not to pursue this feature at this time because {{reason}}. We appreciate your input!`,
  },

  general: {
    thankyou: `Thank you for your feedback! We really appreciate you taking the time to share your thoughts. Your input helps us improve.`,
  },
};

export function generateResponse(
  type: string,
  template: string,
  variables: Record<string, string> = {}
): string {
  let response = responseTemplates[type][template];

  // Replace variables
  for (const [key, value] of Object.entries(variables)) {
    response = response.replace(`{{${key}}}`, value);
  }

  return response;
}
```

## Resources

- [Feedback Collection Best Practices](https://www.nngroup.com/articles/user-feedback/)
- [Discord.js Documentation](https://discord.js.org/)
- [Twitter API v2](https://developer.twitter.com/en/docs/twitter-api)
- [App Store Connect API](https://developer.apple.com/documentation/appstoreconnectapi)
