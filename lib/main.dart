import 'package:flutter/material.dart';
import 'package:flutter_pcd/cube_view.dart';
import 'package:flutter_pcd/main_page.dart';
import 'package:flutter_pcd/pcap_page.dart';
import 'package:flutter_pcd/stream_page.dart';
import 'package:flutter_pcd/store_file_page.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Flutter Demo',
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.deepPurple),
        useMaterial3: true,
      ),
      // home: const MainPage(),
      home: StreamPage(),
      // home: const PcapPage()
      // home: CubePage(),
      // home: const StoreFilePage(),
    );
  }
}
