<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>request to initiate a send to bank operation via IMT</description>
   <name>IMTSendToBank</name>
   <tag></tag>
   <elementGuidId>29dc4c15-f1fb-47b3-a0db-bcc4ff430bca</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;ServiceRequest\&quot;: {\n    \&quot;Header\&quot;: {\n      \&quot;RequestRefID\&quot;: \&quot;Kl45dasdtsad6\&quot;,\n      \&quot;Timestamp\&quot;: \&quot;201304021534l5\&quot;,\n      \&quot;OperationName\&quot;: \&quot;IMTPay\&quot;,\n      \&quot;OperationVersion\&quot;: \&quot;1\&quot;,\n      \&quot;SourceSystem\&quot;: \&quot;SFCAPP\&quot;,\n      \&quot;Credentials\&quot;: {\n        \&quot;UserName\&quot;: \&quot;OdiDance\&quot;,\n        \&quot;Password\&quot;: \&quot;TDN0bTMxbjIwMTcwOTI0MDM0NTM1ODcz\&quot;\n      }\n    },\n    \&quot;Body\&quot;: {\n      \&quot;Data\&quot;: [\n                \n        {\&quot;Key\&quot;: \&quot;countryiso\&quot;, \&quot;Value\&quot;: \&quot;KE\&quot;},\n        {\&quot;Key\&quot;: \&quot;amount\&quot;, \&quot;Value\&quot;: \&quot;10000\&quot;},\n        {\&quot;Key\&quot;: \&quot;currencyiso\&quot;, \&quot;Value\&quot;: \&quot;KES\&quot;},\n        {\&quot;Key\&quot;: \&quot;msisdn\&quot;, \&quot;Value\&quot;: \&quot;254705912645\&quot;},\n                                {\&quot;Key\&quot;: \&quot;feeinquiryrequestid\&quot;, \&quot;Value\&quot;: \&quot;Kenya123\&quot;},\n                                {\&quot;Key\&quot;: \&quot;fname\&quot;, \&quot;Value\&quot;: \&quot;John\&quot;},\n                                {\&quot;Key\&quot;: \&quot;lname\&quot;, \&quot;Value\&quot;: \&quot;Doe\&quot;},\n                                {\&quot;Key\&quot;: \&quot;purposeoffunds\&quot;, \&quot;Value\&quot;: \&quot;fees\&quot;},\n                                {\&quot;Key\&quot;: \&quot;sourceoffunds\&quot;, \&quot;Value\&quot;: \&quot;254705912645\&quot;},\n                                {\&quot;Key\&quot;: \&quot;state\&quot;, \&quot;Value\&quot;: \&quot;Nairobi\&quot;},\n                                {\&quot;Key\&quot;: \&quot;city\&quot;, \&quot;Value\&quot;: \&quot;Nairobi\&quot;},\n                                {\&quot;Key\&quot;: \&quot;question\&quot;, \&quot;Value\&quot;: \&quot;Where is wakanda?\&quot;},\n                                {\&quot;Key\&quot;: \&quot;answer\&quot;, \&quot;Value\&quot;: \&quot; Kenya\&quot;},\n                                {\&quot;Key\&quot;: \&quot;swift\&quot;, \&quot;Value\&quot;: \&quot;65353376\&quot;},\n                                {\&quot;Key\&quot;: \&quot;bankaccountname\&quot;, \&quot;Value\&quot;: \&quot;mybank\&quot;},\n                                {\&quot;Key\&quot;: \&quot;bankaccountnumber\&quot;, \&quot;Value\&quot;: \&quot;123676437\&quot;}\n       \n        \n       ]\n    }\n  }\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://10.184.38.63:15539/IMTSendToBank</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
