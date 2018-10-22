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

payment_response = WS.sendRequest(findTestObject('PaymentTransaction'))

WS.verifyResponseStatusCode(payment_response, 200)

def responseText = payment_response.getResponseText()

def payment_response_object = new groovy.json.JsonSlurper().parseText(responseText)

WS.verifyMatch(payment_response_object.ResponseHeader.ResponseCode, '4000', false, FailureHandling.STOP_ON_FAILURE)

WS.verifyMatch(payment_response_object.ResponseHeader.ResponseMsg, 'Success', false, FailureHandling.STOP_ON_FAILURE)

WS.verifyMatch(payment_response_object.ResponseHeader.DetailedMsg, 'Success, Kindly wait to Enter your MPESA pin.', false, FailureHandling.STOP_ON_FAILURE)
