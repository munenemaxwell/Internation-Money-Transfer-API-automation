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

frequent_recep_resp = WS.sendRequest(findTestObject('IMTFrequentRecipient_negative'))

WS.verifyResponseStatusCode(frequent_recep_resp, 200)

def responseText = frequent_recep_resp.getResponseText()

//println('Response Text: ' + responseText)

def frequent_recep_object = new groovy.json.JsonSlurper().parseText(responseText)

//WS.verifyMatch(frequent_recep_object.ResponseHeader.ResponseCode, '4000', false, FailureHandling.STOP_ON_FAILURE)

WS.verifyMatch(frequent_recep_object.ResponseHeader.ResponseCode, '4006', false, FailureHandling.STOP_ON_FAILURE)
WS.verifyMatch(frequent_recep_object.ResponseHeader.ResponseMsg, 'FAIL', false, FailureHandling.STOP_ON_FAILURE)

WS.verifyMatch(frequent_recep_object.ResponseHeader.DetailedMsg, 'No Records Found', false, FailureHandling.STOP_ON_FAILURE)

