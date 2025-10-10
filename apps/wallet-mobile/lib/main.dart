import 'package:flutter/material.dart';

void main() => runApp(EtridWalletApp());

class EtridWalletApp extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Bloc Banc',
      theme: ThemeData(primarySwatch: Colors.green),
      home: WalletHomePage(),
    );
  }
}

class WalletHomePage extends StatefulWidget {
  @override
  _WalletHomePageState createState() => _WalletHomePageState();
}

class _WalletHomePageState extends State<WalletHomePage> {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: Text('Bloc Banc Wallet')),
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Text('ËTR Balance: 0.00', style: TextStyle(fontSize: 18)),
            SizedBox(height: 20),
            ElevatedButton(onPressed: () {}, child: Text('Send ËTR')),
            ElevatedButton(onPressed: () {}, child: Text('Receive ËTR')),
          ],
        ),
      ),
    );
  }
}
