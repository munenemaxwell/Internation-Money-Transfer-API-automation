import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

//explicit import 
import groovy.json.JsonSlurper
//import to generate unique 
import org.apache.commons.lang.RandomStringUtils

var = WS.sendRequest(findTestObject('IMTQueryCurrencyByCountryISO'))

WS.verifyResponseStatusCode(var, 200)

def responseText = var.getResponseText()

//println('Response Text: ' + responseText)

def jsonObject = new groovy.json.JsonSlurper().parseText(responseText)

//println(jsonObject.ResponseHeader.ResponseCode)

//failure handling to stop on failure so that we can tell if test case passes or fails

WS.verifyMatch(jsonObject.ResponseHeader.ResponseCode, '4000', false, FailureHandling.STOP_ON_FAILURE)

WS.verifyMatch(jsonObject.ResponseHeader.ResponseMsg, 'Success', false, FailureHandling.STOP_ON_FAILURE)

WS.verifyMatch(jsonObject.ResponseHeader.DetailedMsg, 'Transaction Succcess', false, FailureHandling.STOP_ON_FAILURE)






