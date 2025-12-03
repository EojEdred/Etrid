import React, {useState} from 'react';
import {
  View,
  Text,
  StyleSheet,
  TouchableOpacity,
  Dimensions,
} from 'react-native';
import {Camera, useCameraDevices, useCodeScanner} from 'react-native-vision-camera';
import {colors} from '../theme';

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
  const [hasPermission, setHasPermission] = useState(false);
  const devices = useCameraDevices();
  const device = devices.back;

  React.useEffect(() => {
    (async () => {
      const status = await Camera.requestCameraPermission();
      setHasPermission(status === 'granted');
    })();
  }, []);

  const codeScanner = useCodeScanner({
    codeTypes: ['qr', 'ean-13'],
    onCodeScanned: (codes) => {
      if (codes.length > 0 && codes[0].value) {
        onScan(codes[0].value);
      }
    },
  });

  if (!hasPermission) {
    return (
      <View style={styles.container}>
        <Text style={styles.permissionText}>Camera permission required</Text>
      </View>
    );
  }

  if (!device) {
    return (
      <View style={styles.container}>
        <Text style={styles.permissionText}>No camera available</Text>
      </View>
    );
  }

  return (
    <View style={styles.container}>
      <View style={styles.topContent}>
        <Text style={styles.title}>{title}</Text>
        <Text style={styles.subtitle}>
          Position the QR code within the frame
        </Text>
      </View>

      <Camera
        style={styles.camera}
        device={device}
        isActive={true}
        codeScanner={codeScanner}
        torch={flashOn ? 'on' : 'off'}
      />

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
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: colors.background,
  },
  camera: {
    flex: 1,
  },
  topContent: {
    padding: 20,
    alignItems: 'center',
    backgroundColor: 'rgba(0,0,0,0.7)',
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
    backgroundColor: 'rgba(0,0,0,0.7)',
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
  permissionText: {
    color: colors.text,
    fontSize: 18,
    textAlign: 'center',
    marginTop: 100,
  },
});
