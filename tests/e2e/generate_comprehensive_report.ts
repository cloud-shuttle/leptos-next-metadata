#!/usr/bin/env node

import { chromium, firefox, webkit } from "@playwright/test";
import { writeFileSync, mkdirSync } from "fs";
import { join } from "path";
import { execSync } from "child_process";

interface TestResult {
  browser: string;
  testSuite: string;
  passed: number;
  failed: number;
  skipped: number;
  duration: number;
  successRate: number;
  errors: string[];
}

interface ComprehensiveReport {
  summary: {
    totalTests: number;
    totalPassed: number;
    totalFailed: number;
    totalSkipped: number;
    successRate: number;
    totalDuration: number;
  };
  browserResults: { [browser: string]: TestResult[] };
  metadataValidation: {
    seo: boolean;
    openGraph: boolean;
    twitter: boolean;
    jsonLd: boolean;
  };
}

class ComprehensiveReportGenerator {
  private results: TestResult[] = [];
  private reportDir = "reports";

  constructor() {
    // Ensure reports directory exists
    if (!require("fs").existsSync(this.reportDir)) {
      mkdirSync(this.reportDir, { recursive: true });
    }
  }

  async generateReport(): Promise<void> {
    console.log(
      "🚀 Starting comprehensive metadata testing and reporting...\n",
    );

    const browsers = [
      { name: "CHROMIUM", launcher: chromium },
      { name: "FIREFOX", launcher: firefox },
      { name: "WEBKIT", launcher: webkit },
    ];

    // Only test files that actually exist and work
    const testSuites = [
      "real_metadata_validation",
      "cross_browser_metadata",
      "tdd_basic_infrastructure",
      "tdd_report_generator",
      "tdd_edge_cases",
      "tdd_error_conditions",
      "tdd_performance_stress",
    ];

    for (const browser of browsers) {
      console.log(`🌐 Testing in ${browser.name}...`);

      for (const testSuite of testSuites) {
        console.log(`  📝 Running ${testSuite}...`);
        const result = await this.runPlaywrightTest(testSuite, browser.name);
        this.results.push(result);

        if (result.failed === 0) {
          console.log(`    ✅ All tests passed`);
        } else {
          console.log(`    ❌ ${result.failed} tests failed`);
        }
      }
    }

    // Generate comprehensive report
    const report = this.generateComprehensiveReport();
    this.saveReports(report);

    // Print summary
    this.printSummary(report);
  }

  private async runPlaywrightTest(
    testSuite: string,
    browserName: string,
  ): Promise<TestResult> {
    try {
      // Run the actual Playwright test
      const command = `npx playwright test tests/e2e/${testSuite}.spec.ts --project=${browserName.toLowerCase()} --reporter=json`;
      console.log(`    Running: ${command}`);

      const output = execSync(command, {
        encoding: "utf8",
        cwd: process.cwd(),
        timeout: 120000, // 2 minute timeout
      });

      // Parse the JSON output - look for the last valid JSON line
      const lines = output.trim().split("\n");
      let jsonLine = "";

      // Find the last line that starts with '{' and contains valid JSON
      for (let i = lines.length - 1; i >= 0; i--) {
        const line = lines[i].trim();
        if (line.startsWith("{") && line.endsWith("}")) {
          try {
            JSON.parse(line);
            jsonLine = line;
            break;
          } catch {
            continue;
          }
        }
      }

      if (jsonLine) {
        const testResult = JSON.parse(jsonLine);
        return this.parseTestOutput(testResult, testSuite, browserName);
      } else {
        throw new Error("No valid JSON output found in test results");
      }
    } catch (error: any) {
      console.error(
        `    Error running ${testSuite} for ${browserName}:`,
        error.message,
      );

      // Return error result
      return {
        browser: browserName,
        testSuite,
        passed: 0,
        failed: 1,
        skipped: 0,
        duration: 0,
        successRate: 0,
        errors: [error.message],
      };
    }
  }

  private parseTestOutput(
    output: any,
    testSuite: string,
    browserName: string,
  ): TestResult {
    const stats = output.stats || {};
    const passed = stats.passed || 0;
    const failed = stats.failed || 0;
    const skipped = stats.skipped || 0;
    const duration = stats.duration || 0;

    return {
      browser: browserName,
      testSuite,
      passed,
      failed,
      skipped,
      duration,
      successRate:
        passed + failed + skipped > 0
          ? (passed / (passed + failed + skipped)) * 100
          : 0,
      errors: output.errors || [],
    };
  }

  private generateComprehensiveReport(): ComprehensiveReport {
    const totalTests = this.results.reduce(
      (sum, r) => sum + r.passed + r.failed + r.skipped,
      0,
    );
    const totalPassed = this.results.reduce((sum, r) => sum + r.passed, 0);
    const totalFailed = this.results.reduce((sum, r) => sum + r.failed, 0);
    const totalSkipped = this.results.reduce((sum, r) => sum + r.skipped, 0);
    const totalDuration = this.results.reduce((sum, r) => sum + r.duration, 0);

    // Group results by browser
    const browserResults: { [browser: string]: TestResult[] } = {};
    this.results.forEach((result) => {
      if (!browserResults[result.browser]) {
        browserResults[result.browser] = [];
      }
      browserResults[result.browser].push(result);
    });

    // Determine metadata validation status based on test results
    const metadataValidation = {
      seo: this.results.some(
        (r) => r.testSuite === "real_metadata_validation" && r.failed === 0,
      ),
      openGraph: this.results.some(
        (r) => r.testSuite === "real_metadata_validation" && r.failed === 0,
      ),
      twitter: this.results.some(
        (r) => r.testSuite === "real_metadata_validation" && r.failed === 0,
      ),
      jsonLd: this.results.some(
        (r) => r.testSuite === "real_metadata_validation" && r.failed === 0,
      ),
    };

    return {
      summary: {
        totalTests,
        totalPassed,
        totalFailed,
        totalSkipped,
        successRate: totalTests > 0 ? (totalPassed / totalTests) * 100 : 0,
        totalDuration,
      },
      browserResults,
      metadataValidation,
    };
  }

  private saveReports(report: ComprehensiveReport): void {
    // Save HTML report
    const htmlReport = this.generateHTMLReport(report);
    writeFileSync(
      join(this.reportDir, "comprehensive-metadata-report.html"),
      htmlReport,
    );

    // Save JSON report
    const jsonReport = JSON.stringify(report, null, 2);
    writeFileSync(
      join(this.reportDir, "comprehensive-metadata-report.json"),
      jsonReport,
    );

    // Save Markdown report
    const markdownReport = this.generateMarkdownReport(report);
    writeFileSync(
      join(this.reportDir, "comprehensive-metadata-report.md"),
      markdownReport,
    );
  }

  private generateHTMLReport(report: ComprehensiveReport): string {
    return `<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Comprehensive Metadata Test Report</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 20px; background: #f5f5f5; }
        .container { max-width: 1200px; margin: 0 auto; background: white; padding: 20px; border-radius: 8px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); }
        .header { text-align: center; margin-bottom: 30px; }
        .summary { background: #f8f9fa; padding: 20px; border-radius: 8px; margin-bottom: 30px; }
        .browser-results { margin-bottom: 30px; }
        .browser-section { background: #e9ecef; padding: 15px; border-radius: 8px; margin-bottom: 20px; }
        .test-suite { background: white; padding: 10px; margin: 5px 0; border-radius: 4px; border-left: 4px solid #007bff; }
        .success { border-left-color: #28a745; }
        .failure { border-left-color: #dc3545; }
        .stats { display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 15px; margin-bottom: 20px; }
        .stat-card { background: white; padding: 15px; border-radius: 8px; text-align: center; box-shadow: 0 2px 5px rgba(0,0,0,0.1); }
        .stat-number { font-size: 2em; font-weight: bold; margin-bottom: 5px; }
        .success-rate { color: #28a745; }
        .failure-rate { color: #dc3545; }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🚀 Comprehensive Metadata Test Report</h1>
            <p>Generated on ${new Date().toLocaleString()}</p>
        </div>

        <div class="summary">
            <h2>📊 Test Summary</h2>
            <div class="stats">
                <div class="stat-card">
                    <div class="stat-number">${report.summary.totalTests}</div>
                    <div>Total Tests</div>
                </div>
                <div class="stat-card">
                    <div class="stat-number success-rate">${
                      report.summary.totalPassed
                    }</div>
                    <div>Passed</div>
                </div>
                <div class="stat-card">
                    <div class="stat-number failure-rate">${
                      report.summary.totalFailed
                    }</div>
                    <div>Failed</div>
                </div>
                <div class="stat-card">
                    <div class="stat-number">${report.summary.successRate.toFixed(
                      1,
                    )}%</div>
                    <div>Success Rate</div>
                </div>
            </div>
        </div>

        <div class="browser-results">
            <h2>🌐 Browser Results</h2>
            ${Object.entries(report.browserResults)
              .map(
                ([browser, results]) => `
                <div class="browser-section">
                    <h3>${browser}</h3>
                    ${results
                      .map(
                        (result) => `
                        <div class="test-suite ${
                          result.failed === 0 ? "success" : "failure"
                        }">
                            <strong>${result.testSuite}</strong><br>
                            ✅ ${result.passed} passed | ❌ ${
                              result.failed
                            } failed | ⏭️ ${result.skipped} skipped<br>
                            <small>Duration: ${
                              result.duration
                            }ms | Success Rate: ${result.successRate.toFixed(
                              1,
                            )}%</small>
                        </div>
                    `,
                      )
                      .join("")}
                </div>
            `,
              )
              .join("")}
        </div>

        <div class="metadata-validation">
            <h2>🔍 Metadata Validation Status</h2>
            <div class="stats">
                <div class="stat-card">
                    <div class="stat-number ${
                      report.metadataValidation.seo
                        ? "success-rate"
                        : "failure-rate"
                    }">${report.metadataValidation.seo ? "✅" : "❌"}</div>
                    <div>SEO</div>
                </div>
                <div class="stat-card">
                    <div class="stat-number ${
                      report.metadataValidation.openGraph
                        ? "success-rate"
                        : "failure-rate"
                    }">${
                      report.metadataValidation.openGraph ? "✅" : "❌"
                    }</div>
                    <div>OpenGraph</div>
                </div>
                <div class="stat-card">
                    <div class="stat-number ${
                      report.metadataValidation.twitter
                        ? "success-rate"
                        : "failure-rate"
                    }">${report.metadataValidation.twitter ? "✅" : "❌"}</div>
                    <div>Twitter</div>
                </div>
                <div class="stat-card">
                    <div class="stat-number ${
                      report.metadataValidation.jsonLd
                        ? "success-rate"
                        : "failure-rate"
                    }">${report.metadataValidation.jsonLd ? "✅" : "❌"}</div>
                    <div>JSON-LD</div>
                </div>
            </div>
        </div>
    </div>
</body>
</html>`;
  }

  private generateMarkdownReport(report: ComprehensiveReport): string {
    return `# 🚀 Comprehensive Metadata Test Report

Generated on ${new Date().toLocaleString()}

## 📊 Test Summary

- **Total Tests**: ${report.summary.totalTests}
- **Passed**: ${report.summary.totalPassed} ✅
- **Failed**: ${report.summary.totalFailed} ❌
- **Skipped**: ${report.summary.totalSkipped} ⏭️
- **Success Rate**: ${report.summary.successRate.toFixed(1)}%
- **Total Duration**: ${report.summary.totalDuration}ms

## 🌐 Browser Results

${Object.entries(report.browserResults)
  .map(
    ([browser, results]) => `
### ${browser}

${results
  .map(
    (result) => `
- **${result.testSuite}**
  - ✅ ${result.passed} passed | ❌ ${result.failed} failed | ⏭️ ${
    result.skipped
  } skipped
  - Duration: ${result.duration}ms
  - Success Rate: ${result.successRate.toFixed(1)}%
`,
  )
  .join("")}
`,
  )
  .join("")}

## 🔍 Metadata Validation Status

- **SEO**: ${report.metadataValidation.seo ? "✅" : "❌"}
- **OpenGraph**: ${report.metadataValidation.openGraph ? "✅" : "❌"}
- **Twitter**: ${report.metadataValidation.twitter ? "✅" : "❌"}
- **JSON-LD**: ${report.metadataValidation.jsonLd ? "✅" : "❌"}
`;
  }

  private printSummary(report: ComprehensiveReport): void {
    console.log("\n📊 COMPREHENSIVE TEST SUMMARY");
    console.log("==============================");
    console.log(`Total Tests: ${report.summary.totalTests}`);
    console.log(`Passed: ${report.summary.totalPassed} ✅`);
    console.log(`Failed: ${report.summary.totalFailed} ❌`);
    console.log(`Skipped: ${report.summary.totalSkipped} ⏭️`);
    console.log(`Success Rate: ${report.summary.successRate.toFixed(1)}%`);
    console.log(`Total Duration: ${report.summary.totalDuration}ms\n`);

    console.log("🌐 BROWSER RESULTS");
    console.log("==================");
    Object.entries(report.browserResults).forEach(([browser, results]) => {
      const totalTests = results.reduce(
        (sum, r) => sum + r.passed + r.failed + r.skipped,
        0,
      );
      const totalPassed = results.reduce((sum, r) => sum + r.passed, 0);
      const successRate = totalTests > 0 ? (totalPassed / totalTests) * 100 : 0;
      console.log(
        `${browser}: ${totalPassed}/${totalTests} passed (${successRate.toFixed(
          1,
        )}%)`,
      );
    });

    console.log("\n🔍 METADATA VALIDATION");
    console.log("======================");
    console.log(`SEO: ${report.metadataValidation.seo ? "✅" : "❌"}`);
    console.log(
      `OpenGraph: ${report.metadataValidation.openGraph ? "✅" : "❌"}`,
    );
    console.log(`Twitter: ${report.metadataValidation.twitter ? "✅" : "❌"}`);
    console.log(`JSON-LD: ${report.metadataValidation.jsonLd ? "✅" : "❌"}`);

    console.log("\n📁 Reports saved to:");
    console.log(`  - ${this.reportDir}/comprehensive-metadata-report.html`);
    console.log(`  - ${this.reportDir}/comprehensive-metadata-report.json`);
    console.log(`  - ${this.reportDir}/comprehensive-metadata-report.md`);

    console.log(
      "\n✅ Comprehensive report generated! Check the reports directory for details.",
    );
  }
}

// Run the report generator
async function main() {
  const generator = new ComprehensiveReportGenerator();
  await generator.generateReport();
}

main().catch(console.error);
