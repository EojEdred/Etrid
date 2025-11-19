import React, {useState} from 'react';
import {
  View,
  Text,
  StyleSheet,
  TouchableOpacity,
  Dimensions,
} from 'react-native';
import QRCodeScanner from 'react-native-qrcode-scanner';
import {colors} from '@/theme';

interface QRScannerProps {
  onScan: (data: string) => void;
  onClose: () => void;
  title?: string;
}

const {width, height} = Dimensions.get('window');

export default function QRScanner({
  onScan,
  onClose,
  title = 'Scan QR Code',
}: QRScannerProps) {
  const [flashOn, setFlashOn] = useState(false);

  const handleScan = (e: any) => {
    if (e.data) {
      onScan(e.data);
    }
  };

  return (
    <View style={styles.container}>
      <QRCodeScanner
        onRead={handleScan}
        flashMode={
          flashOn
            ? RNCamera.Constants.FlashMode.torch
            : RNCamera.Constants.FlashMode.off
        }
        topContent={
          <View style={styles.topContent}>
            <Text style={styles.title}>{title}</Text>
            <Text style={styles.subtitle}>
              Position the QR code within the frame
            </Text>
          </View>
        }
        bottomContent={
          <View style={styles.bottomContent}>
            <TouchableOpacity
              style={styles.flashButton}
              onPress={() => setFlashOn(!flashOn)}>
              <Text style={styles.flashText}>
                {flashOn ? 'Flash Off' : 'Flash On'}
              </Text>
            </TouchableOpacity>
            <TouchableOpacity style={styles.cancelButton} onPress={onClose}>
              <Text style={styles.cancelText}>Cancel</Text>
            </TouchableOpacity>
          </View>
        }
        cameraStyle={styles.camera}
        containerStyle={styles.scannerContainer}
      />
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: colors.background,
  },
  scannerContainer: {
    flex: 1,
  },
  camera: {
    height: height,
  },
  topContent: {
    padding: 20,
    alignItems: 'center',
  },
  title: {
    fontSize: 24,
    fontWeight: 'bold',
    color: colors.text,
    marginBottom: 8,
  },
  subtitle: {
    fontSize: 14,
    color: colors.textSecondary,
    textAlign: 'center',
  },
  bottomContent: {
    padding: 20,
    flexDirection: 'row',
    justifyContent: 'space-around',
    width: width,
  },
  flashButton: {
    backgroundColor: colors.surface,
    paddingHorizontal: 24,
    paddingVertical: 12,
    borderRadius: 12,
  },
  flashText: {
    color: colors.text,
    fontSize: 16,
    fontWeight: '600',
  },
  cancelButton: {
    backgroundColor: colors.error,
    paddingHorizontal: 24,
    paddingVertical: 12,
    borderRadius: 12,
  },
  cancelText: {
    color: colors.text,
    fontSize: 16,
    fontWeight: '600',
  },
});
