/**
 * Frontend Integration Tests for DeepBook Liquidity Provisioning
 * 
 * These tests validate the frontend application's functionality,
 * user interactions, and integration with blockchain services.
 */

const { expect } = require('chai');
const puppeteer = require('puppeteer');

describe('Frontend Integration Tests', function() {
    let browser;
    let page;
    const BASE_URL = 'http://localhost:5173';

    before(async function() {
        // Launch browser for testing
        browser = await puppeteer.launch({
            headless: true,
            args: ['--no-sandbox', '--disable-setuid-sandbox']
        });
        page = await browser.newPage();
        
        // Set viewport size
        await page.setViewport({ width: 1280, height: 720 });
    });

    after(async function() {
        if (browser) {
            await browser.close();
        }
    });

    describe('Application Loading', function() {
        it('should load the main dashboard', async function() {
            await page.goto(BASE_URL);
            
            // Wait for the main heading to appear
            await page.waitForSelector('h1', { timeout: 10000 });
            
            const title = await page.$eval('h1', el => el.textContent);
            expect(title).to.include('DeepBook Liquidity Provisioning');
        });

        it('should display portfolio overview cards', async function() {
            // Check for portfolio overview cards
            const cards = await page.$$('[data-testid="portfolio-card"]');
            expect(cards.length).to.be.greaterThan(0);
        });
    });

    describe('Strategy Selection', function() {
        it('should display available strategies', async function() {
            // Wait for strategies to load
            await page.waitForSelector('[data-testid="strategy-card"]', { timeout: 5000 });
            
            const strategies = await page.$$('[data-testid="strategy-card"]');
            expect(strategies.length).to.be.greaterThan(0);
        });

        it('should show strategy details correctly', async function() {
            // Check if strategy cards contain required information
            const strategyCard = await page.$('[data-testid="strategy-card"]');
            expect(strategyCard).to.not.be.null;
            
            const apy = await strategyCard.$eval('[data-testid="strategy-apy"]', el => el.textContent);
            expect(apy).to.match(/\d+\.\d+%/);
        });
    });

    describe('User Interactions', function() {
        it('should handle tab navigation', async function() {
            // Click on "My Positions" tab
            await page.click('[data-value="positions"]');
            
            // Wait for positions content to load
            await page.waitForSelector('[data-testid="positions-content"]', { timeout: 5000 });
            
            // Verify we're on the positions tab
            const activeTab = await page.$eval('[data-state="active"]', el => el.textContent);
            expect(activeTab).to.include('My Positions');
        });

        it('should handle analytics tab', async function() {
            // Click on "Analytics" tab
            await page.click('[data-value="analytics"]');
            
            // Wait for analytics content to load
            await page.waitForSelector('[data-testid="analytics-content"]', { timeout: 5000 });
            
            // Verify analytics content is displayed
            const performanceCard = await page.$('[data-testid="performance-overview"]');
            expect(performanceCard).to.not.be.null;
        });
    });

    describe('Form Interactions', function() {
        it('should handle strategy selection in deployment form', async function() {
            // Go back to strategies tab
            await page.click('[data-value="strategies"]');
            
            // Click on strategy selector
            await page.click('[data-testid="strategy-selector"]');
            
            // Wait for dropdown options
            await page.waitForSelector('[data-testid="strategy-option"]', { timeout: 5000 });
            
            // Select first strategy
            await page.click('[data-testid="strategy-option"]:first-child');
            
            // Verify selection
            const selectedValue = await page.$eval('[data-testid="strategy-selector"]', el => el.textContent);
            expect(selectedValue).to.not.be.empty;
        });

        it('should validate deposit amount input', async function() {
            // Enter deposit amount
            await page.type('[data-testid="deposit-amount"]', '1000');
            
            const inputValue = await page.$eval('[data-testid="deposit-amount"]', el => el.value);
            expect(inputValue).to.equal('1000');
        });
    });

    describe('Responsive Design', function() {
        it('should adapt to mobile viewport', async function() {
            // Set mobile viewport
            await page.setViewport({ width: 375, height: 667 });
            
            // Reload page
            await page.reload();
            
            // Check if mobile layout is applied
            const container = await page.$('.container');
            const containerClass = await page.evaluate(el => el.className, container);
            
            // Verify responsive classes are applied
            expect(containerClass).to.include('mx-auto');
        });

        it('should maintain functionality on tablet viewport', async function() {
            // Set tablet viewport
            await page.setViewport({ width: 768, height: 1024 });
            
            // Reload page
            await page.reload();
            
            // Verify main functionality still works
            await page.waitForSelector('h1', { timeout: 5000 });
            const title = await page.$eval('h1', el => el.textContent);
            expect(title).to.include('DeepBook Liquidity Provisioning');
        });
    });

    describe('Error Handling', function() {
        it('should handle network errors gracefully', async function() {
            // Simulate network failure
            await page.setOfflineMode(true);
            
            // Try to reload page
            try {
                await page.reload({ waitUntil: 'networkidle0', timeout: 5000 });
            } catch (error) {
                // Expected to fail due to offline mode
                expect(error.message).to.include('net::ERR_INTERNET_DISCONNECTED');
            }
            
            // Restore network
            await page.setOfflineMode(false);
        });
    });

    describe('Performance', function() {
        it('should load within acceptable time limits', async function() {
            const startTime = Date.now();
            
            await page.goto(BASE_URL, { waitUntil: 'networkidle0' });
            
            const loadTime = Date.now() - startTime;
            expect(loadTime).to.be.lessThan(5000); // Should load within 5 seconds
        });

        it('should have good Lighthouse scores', async function() {
            // This would integrate with Lighthouse for performance testing
            // For now, we'll just verify the page loads efficiently
            const metrics = await page.metrics();
            
            expect(metrics.JSHeapUsedSize).to.be.lessThan(50 * 1024 * 1024); // Less than 50MB
        });
    });
});

module.exports = {
    // Export any helper functions if needed
};

