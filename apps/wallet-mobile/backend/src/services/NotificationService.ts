import axios from 'axios';
import config from '../config';
import logger from '../utils/logger';
import db from '../database/client';
import { Notification } from '../types';

class NotificationService {
  /**
   * Send notification to user
   */
  async sendNotification(params: {
    userId: string;
    type: string;
    title: string;
    body: string;
    data?: any;
    channels?: ('push' | 'email' | 'sms')[];
  }): Promise<void> {
    try {
      const { userId, type, title, body, data, channels = ['push'] } = params;

      // Store notification in database
      const result = await db.query(
        `INSERT INTO notifications
           (user_id, notification_type, title, body, data, sent_via)
         VALUES ($1, $2, $3, $4, $5, $6)
         RETURNING *`,
        [userId, type, title, body, JSON.stringify(data || {}), channels]
      );

      const notification = result.rows[0];

      // Send via requested channels
      const sendPromises = [];

      if (channels.includes('push')) {
        sendPromises.push(this.sendPushNotification(userId, title, body, data));
      }

      if (channels.includes('email')) {
        sendPromises.push(this.sendEmailNotification(userId, title, body));
      }

      if (channels.includes('sms')) {
        sendPromises.push(this.sendSMSNotification(userId, body));
      }

      await Promise.allSettled(sendPromises);

      // Mark as sent
      await db.query(
        `UPDATE notifications SET sent_at = NOW() WHERE id = $1`,
        [notification.id]
      );

      logger.info('Notification sent', {
        notificationId: notification.id,
        userId,
        type,
        channels,
      });
    } catch (error: any) {
      logger.error('Error sending notification', {
        userId: params.userId,
        error: error.message,
      });
    }
  }

  /**
   * Send push notification via Expo
   */
  private async sendPushNotification(
    userId: string,
    title: string,
    body: string,
    data?: any
  ): Promise<void> {
    try {
      // Get user's push tokens
      const result = await db.query(
        `SELECT preferences->'push_tokens' as push_tokens FROM users WHERE id = $1`,
        [userId]
      );

      const pushTokens = result.rows[0]?.push_tokens || [];

      if (pushTokens.length === 0) {
        logger.warn('No push tokens found for user', { userId });
        return;
      }

      // Send to Expo Push API
      await axios.post(
        'https://exp.host/--/api/v2/push/send',
        {
          to: pushTokens,
          title,
          body,
          data,
          sound: 'default',
          priority: 'high',
        },
        {
          headers: {
            'Content-Type': 'application/json',
            Authorization: `Bearer ${config.notifications.expo.accessToken}`,
          },
        }
      );

      logger.info('Push notification sent', { userId, tokenCount: pushTokens.length });
    } catch (error: any) {
      logger.error('Error sending push notification', {
        userId,
        error: error.message,
      });
    }
  }

  /**
   * Send email notification via SendGrid
   */
  private async sendEmailNotification(
    userId: string,
    title: string,
    body: string
  ): Promise<void> {
    try {
      // Get user email
      const result = await db.query(
        `SELECT email FROM users WHERE id = $1`,
        [userId]
      );

      const email = result.rows[0]?.email;

      if (!email) {
        logger.warn('No email found for user', { userId });
        return;
      }

      // Send via SendGrid
      await axios.post(
        'https://api.sendgrid.com/v3/mail/send',
        {
          personalizations: [
            {
              to: [{ email }],
              subject: title,
            },
          ],
          from: {
            email: config.notifications.sendgrid.fromEmail,
            name: 'Ëtrid Wallet',
          },
          content: [
            {
              type: 'text/html',
              value: `<html><body><h2>${title}</h2><p>${body}</p></body></html>`,
            },
          ],
        },
        {
          headers: {
            Authorization: `Bearer ${config.notifications.sendgrid.apiKey}`,
            'Content-Type': 'application/json',
          },
        }
      );

      logger.info('Email notification sent', { userId, email });
    } catch (error: any) {
      logger.error('Error sending email notification', {
        userId,
        error: error.message,
      });
    }
  }

  /**
   * Send SMS notification via Twilio
   */
  private async sendSMSNotification(userId: string, body: string): Promise<void> {
    try {
      // Get user phone
      const result = await db.query(
        `SELECT phone FROM users WHERE id = $1`,
        [userId]
      );

      const phone = result.rows[0]?.phone;

      if (!phone) {
        logger.warn('No phone found for user', { userId });
        return;
      }

      // Send via Twilio
      const twilioUrl = `https://api.twilio.com/2010-04-01/Accounts/${config.notifications.twilio.accountSid}/Messages.json`;

      await axios.post(
        twilioUrl,
        new URLSearchParams({
          To: phone,
          From: config.notifications.twilio.phoneNumber,
          Body: body,
        }),
        {
          auth: {
            username: config.notifications.twilio.accountSid,
            password: config.notifications.twilio.authToken,
          },
        }
      );

      logger.info('SMS notification sent', { userId, phone });
    } catch (error: any) {
      logger.error('Error sending SMS notification', {
        userId,
        error: error.message,
      });
    }
  }

  /**
   * Notify on transaction confirmation
   */
  async notifyTransactionConfirmed(userId: string, txHash: string): Promise<void> {
    await this.sendNotification({
      userId,
      type: 'tx_confirmed',
      title: 'Transaction Confirmed',
      body: `Your transaction has been confirmed on the blockchain.`,
      data: { txHash },
      channels: ['push'],
    });
  }

  /**
   * Notify on proposal voting ending soon
   */
  async notifyProposalEndingSoon(userId: string, proposalId: number): Promise<void> {
    await this.sendNotification({
      userId,
      type: 'proposal_ending',
      title: 'Proposal Voting Ends Soon',
      body: `Voting for proposal #${proposalId} ends in 24 hours. Cast your vote now!`,
      data: { proposalId },
      channels: ['push', 'email'],
    });
  }

  /**
   * Notify on rewards received
   */
  async notifyRewardsReceived(
    userId: string,
    amount: string,
    validator: string
  ): Promise<void> {
    await this.sendNotification({
      userId,
      type: 'rewards_received',
      title: 'Staking Rewards Received',
      body: `You received ${amount} ETR in staking rewards from ${validator}.`,
      data: { amount, validator },
      channels: ['push'],
    });
  }

  /**
   * Notify on ATM withdrawal ready
   */
  async notifyATMWithdrawalReady(
    userId: string,
    withdrawalCode: string,
    amount: number
  ): Promise<void> {
    await this.sendNotification({
      userId,
      type: 'atm_ready',
      title: 'ATM Withdrawal Ready',
      body: `Your withdrawal of $${amount} is ready. Use code: ${withdrawalCode}`,
      data: { withdrawalCode, amount },
      channels: ['push', 'sms'],
    });
  }
}

export default new NotificationService();
