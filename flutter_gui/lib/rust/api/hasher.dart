// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.28.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

Future<CalculateHashesResult> hasherProcess(
        {required String directory,
        required String hashAlgorithm,
        required String csvOutputFilename,
        dynamic hint}) =>
    RustLib.instance.api.hasherProcess(
        directory: directory,
        hashAlgorithm: hashAlgorithm,
        csvOutputFilename: csvOutputFilename,
        hint: hint);

List<String> availableHashingAlgorithms({dynamic hint}) =>
    RustLib.instance.api.availableHashingAlgorithms(hint: hint);

class CalculateHashesResult {
  final double elapsedTimeSecs;
  final int processedFilesCount;

  const CalculateHashesResult({
    required this.elapsedTimeSecs,
    required this.processedFilesCount,
  });

  @override
  int get hashCode => elapsedTimeSecs.hashCode ^ processedFilesCount.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is CalculateHashesResult &&
          runtimeType == other.runtimeType &&
          elapsedTimeSecs == other.elapsedTimeSecs &&
          processedFilesCount == other.processedFilesCount;
}
