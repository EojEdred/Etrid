# Firebase Configuration Checklist

Complete checklist for setting up Firebase across all Ëtrid Wallet platforms.

## 🔥 Firebase Console Setup

### Project Creation
- [ ] Create Firebase project named "etrid-wallet"
- [ ] Enable Google Analytics
- [ ] Note down Project ID
- [ ] Set up billing (if needed for production features)

### Web App (PWA)
- [ ] Add Web app to Firebase project
- [ ] App nickname: "Ëtrid Wallet PWA"
- [ ] Copy Firebase config object
- [ ] Save config to `.env.local`
- [ ] Generate VAPID key for FCM
- [ ] Save VAPID key to `.env.local`

### iOS App
- [ ] Add iOS app to Firebase project
- [ ] Bundle ID: `com.etrid.wallet`
- [ ] Download `GoogleService-Info.plist`
- [ ] Place file in `ios/EtridWallet/`
- [ ] Add file to Xcode project

### Android App
- [ ] Add Android app to Firebase project
- [ ] Package name: `com.etrid.wallet`
- [ ] Download `google-services.json`
- [ ] Place file in `android/app/`
- [ ] Verify file is included in build

### Cloud Messaging
- [ ] Enable Cloud Messaging API
- [ ] Generate VAPID key pair (Web)
- [ ] Copy Server key for backend
- [ ] Store Server key securely (not in Git!)
- [ ] Configure notification settings

### Analytics
- [ ] Verify Analytics is enabled
- [ ] Enable DebugView
- [ ] Configure custom events (optional)
- [ ] Set up conversion events (optional)
- [ ] Test event logging

### Crashlytics
- [ ] Enable Crashlytics
- [ ] Complete iOS setup
- [ ] Complete Android setup
- [ ] Test crash reporting
- [ ] Upload debug symbols

### Service Account
- [ ] Generate service account key (JSON)
- [ ] Download and store securely
- [ ] Add to `.gitignore`
- [ ] Configure backend to use service account

## 🌐 PWA/Web Setup

### Dependencies
- [ ] Install Firebase SDK: `npm install firebase`
- [ ] Verify version (latest stable)
- [ ] Update package.json

### Configuration Files
- [ ] Create `lib/firebase/config.ts`
- [ ] Create `lib/firebase/analytics.ts`
- [ ] Create `lib/firebase/crashlytics.ts`
- [ ] Create `public/firebase-messaging-sw.js`
- [ ] Update service worker registration

### Environment Variables
- [ ] Create `.env.local` from `.env.example`
- [ ] Fill in all Firebase config values
- [ ] Add `.env.local` to `.gitignore`
- [ ] Verify environment variables load correctly

### Service Worker
- [ ] Add `firebase-messaging-sw.js` to `/public`
- [ ] Update Firebase config in service worker
- [ ] Register service worker in app
- [ ] Test service worker registration
- [ ] Verify notification permission request

### Testing
- [ ] Build app: `npm run build`
- [ ] Start production server: `npm start`
- [ ] Open in browser (HTTPS required)
- [ ] Accept notification permission
- [ ] Copy FCM token from console
- [ ] Send test notification from Firebase Console
- [ ] Verify notification appears (foreground)
- [ ] Verify notification appears (background)
- [ ] Test notification click handling
- [ ] Verify analytics events in DebugView

## 📱 iOS Setup

### Xcode Configuration
- [ ] Open project in Xcode
- [ ] Add `GoogleService-Info.plist` to project
- [ ] Verify file is in target membership
- [ ] Enable Push Notifications capability
- [ ] Enable Background Modes → Remote notifications

### Dependencies
- [ ] Install CocoaPods: `cd ios && pod install`
- [ ] Add Firebase pods to Podfile
- [ ] Run `pod install`
- [ ] Verify pods are installed

### Code Integration
- [ ] Import Firebase in `AppDelegate.mm`
- [ ] Initialize Firebase in `didFinishLaunchingWithOptions`
- [ ] Add notification delegate methods
- [ ] Implement token refresh handling
- [ ] Create `FirebaseService.ts` in React Native

### APNs Setup (Apple Push Notification service)
- [ ] Create APNs key in Apple Developer Console
- [ ] Download APNs key (.p8 file)
- [ ] Upload APNs key to Firebase Console
- [ ] Note Key ID and Team ID
- [ ] Test APNs connection

### Testing
- [ ] Build app on physical device (notifications don't work on simulator)
- [ ] Request notification permission
- [ ] Copy FCM token from logs
- [ ] Send test notification from Firebase Console
- [ ] Verify notification appears on lock screen
- [ ] Test notification tap handling
- [ ] Test background notifications
- [ ] Verify analytics events
- [ ] Test crash reporting

## 🤖 Android Setup

### Gradle Configuration
- [ ] Add Google Services classpath to project `build.gradle`
- [ ] Add Firebase Crashlytics classpath to project `build.gradle`
- [ ] Apply Google Services plugin in app `build.gradle`
- [ ] Apply Crashlytics plugin in app `build.gradle`
- [ ] Add Firebase BOM to dependencies
- [ ] Add specific Firebase dependencies

### Configuration Files
- [ ] Verify `google-services.json` is in `app/` directory
- [ ] Ensure file is not in `.gitignore`
- [ ] Check package name matches Firebase Console

### Code Integration
- [ ] Create `FirebaseService.ts` in React Native
- [ ] Request notification permission
- [ ] Handle FCM token registration
- [ ] Implement notification listeners

### Android Manifest
- [ ] Add notification icon (optional)
- [ ] Add notification color (optional)
- [ ] Configure notification channels

### Testing
- [ ] Build debug APK: `cd android && ./gradlew assembleDebug`
- [ ] Install on physical device
- [ ] Request notification permission
- [ ] Copy FCM token from logs
- [ ] Send test notification from Firebase Console
- [ ] Verify notification appears
- [ ] Test notification tap handling
- [ ] Test background notifications
- [ ] Verify analytics events
- [ ] Test crash reporting

## 🔧 Backend Setup

### Service Account
- [ ] Download service account key from Firebase
- [ ] Store in secure location (not in repository)
- [ ] Add path to environment variable
- [ ] Test service account authentication

### FCM Server
- [ ] Create endpoint for token registration
- [ ] Create endpoint for sending notifications
- [ ] Implement topic subscription (optional)
- [ ] Add rate limiting
- [ ] Add error handling
- [ ] Test notification sending

### Database
- [ ] Create table/collection for FCM tokens
- [ ] Store user ID → FCM token mapping
- [ ] Handle token refresh
- [ ] Clean up invalid tokens

## 🧪 Testing & Validation

### Functionality Tests
- [ ] Push notifications work on all platforms
- [ ] Notifications appear in foreground
- [ ] Notifications appear in background
- [ ] Notification click opens app
- [ ] Deep linking works (optional)
- [ ] Analytics events are logged
- [ ] Crash reports are received

### Performance Tests
- [ ] App startup time not affected
- [ ] Battery usage is acceptable
- [ ] Network usage is minimal
- [ ] No memory leaks

### User Experience
- [ ] Permission requests are timely
- [ ] Notifications are relevant
- [ ] Notification UI is clear
- [ ] Settings are accessible

## 📊 Monitoring & Maintenance

### Firebase Console
- [ ] Check Analytics dashboard daily
- [ ] Review Crashlytics reports
- [ ] Monitor Cloud Messaging quota
- [ ] Check performance metrics

### Alerts
- [ ] Set up alerts for crashes
- [ ] Set up alerts for quota limits
- [ ] Set up alerts for errors

### Documentation
- [ ] Document Firebase setup process
- [ ] Document troubleshooting steps
- [ ] Document notification format
- [ ] Update README with Firebase info

## 🔒 Security

### API Keys
- [ ] Restrict API keys by domain/app
- [ ] Restrict API keys by API
- [ ] Never commit keys to Git
- [ ] Rotate keys periodically

### Security Rules
- [ ] Set up Firestore security rules (if using)
- [ ] Set up Storage security rules (if using)
- [ ] Test security rules
- [ ] Review rules regularly

### Privacy
- [ ] Add privacy policy
- [ ] Disclose data collection
- [ ] Provide opt-out options
- [ ] Comply with GDPR/CCPA

## 🚀 Production Deployment

### Pre-deployment
- [ ] Test all features in staging
- [ ] Review all configurations
- [ ] Check environment variables
- [ ] Verify API key restrictions

### Deployment
- [ ] Deploy PWA to production
- [ ] Submit iOS app to App Store
- [ ] Submit Android app to Play Store
- [ ] Monitor initial rollout

### Post-deployment
- [ ] Monitor crash reports
- [ ] Monitor analytics
- [ ] Check notification delivery rates
- [ ] Gather user feedback

## ✅ Final Verification

- [ ] All platforms successfully configured
- [ ] All tests passing
- [ ] Documentation complete
- [ ] Team trained on Firebase usage
- [ ] Monitoring dashboards set up
- [ ] Backup and recovery plan in place

---

## Notes

- Use this checklist for initial setup and periodic audits
- Update checklist as Firebase features evolve
- Share with team members involved in setup
- Keep a copy of completed checklist for reference

## Status Legend

- [ ] Not started
- [⏳] In progress
- [✅] Complete
- [❌] Blocked/Issue
- [⏭️] Skipped (not needed)

---

**Last Updated**: 2025-11-19
**Next Review**: Before production deployment
