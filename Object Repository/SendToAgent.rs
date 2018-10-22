<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>SendToAgent</name>
   <tag></tag>
   <elementGuidId>9e2e5828-d0c6-4a72-b123-9b0fb971b861</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n   \&quot;ServiceRequest\&quot;: {\n      \&quot;Header\&quot;: {\n         \&quot;RequestRefID\&quot;: \&quot;eDkfjdddasdafFeerfsrdfdaserr5ffv\&quot;,\n         \&quot;Timestamp\&quot;: \&quot;201304021534l5\&quot;,\n         \&quot;OperationName\&quot;: \&quot;IMTPay\&quot;,\n         \&quot;OperationVersion\&quot;: \&quot;1\&quot;,\n         \&quot;SourceSystem\&quot;: \&quot;SFCAPP\&quot;,\n         \&quot;Credentials\&quot;: {\n            \&quot;UserName\&quot;: \&quot;OdiDance\&quot;,\n            \&quot;Password\&quot;: \&quot;TDN0bTMxbjIwMTcwOTI0MDM0NTM1ODcz\&quot;\n         }\n      },\n      \&quot;Body\&quot;: {\n         \&quot;Data\&quot;: [\n            {\n               \&quot;Key\&quot;: \&quot;countryiso\&quot;,\n               \&quot;Value\&quot;: \&quot;KE\&quot;\n            },\n            {\n               \&quot;Key\&quot;: \&quot;amount\&quot;,\n               \&quot;Value\&quot;: \&quot;1000\&quot;\n            },\n            {\n               \&quot;Key\&quot;: \&quot;currencyiso\&quot;,\n               \&quot;Value\&quot;: \&quot;KES\&quot;\n            },\n            {\n               \&quot;Key\&quot;: \&quot;msisdn\&quot;,\n               \&quot;Value\&quot;: \&quot;254705912645\&quot;\n            },\n            {\n               \&quot;Key\&quot;: \&quot;feeinquiryrequestid\&quot;,\n               \&quot;Value\&quot;: \&quot;emmanuel\&quot;\n            },\n            {\n               \&quot;Key\&quot;: \&quot;fname\&quot;,\n               \&quot;Value\&quot;: \&quot;John\&quot;\n            },\n            {\n               \&quot;Key\&quot;: \&quot;lname\&quot;,\n               \&quot;Value\&quot;: \&quot;Doe\&quot;\n            },\n            {\n               \&quot;Key\&quot;: \&quot;mname\&quot;,\n               \&quot;Value\&quot;: \&quot;Doe\&quot;\n            },\n            {\n               \&quot;Key\&quot;: \&quot;purposeoffunds\&quot;,\n               \&quot;Value\&quot;: \&quot;fees\&quot;\n            },\n            {\n               \&quot;Key\&quot;: \&quot;sourceoffunds\&quot;,\n               \&quot;Value\&quot;: \&quot;254705912645\&quot;\n            },\n            {\n               \&quot;Key\&quot;: \&quot;state\&quot;,\n               \&quot;Value\&quot;: \&quot;Nairobi\&quot;\n            },\n            {\n               \&quot;Key\&quot;: \&quot;city\&quot;,\n               \&quot;Value\&quot;: \&quot;Nairobi\&quot;\n            },\n            {\n               \&quot;Key\&quot;: \&quot;question\&quot;,\n               \&quot;Value\&quot;: \&quot;Where is wakanda?\&quot;\n            },\n            {\n               \&quot;Key\&quot;: \&quot;answer\&quot;,\n               \&quot;Value\&quot;: \&quot; Kenya\&quot;\n            }\n            \n         ]\n      }\n   }\n}&quot;,
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
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic T2RpRGFuY2U6VEROMGJUTXhiakl3TVRjd09USTBNRE0wTlRNMU9EY3o=</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://10.184.38.63:15541/Agent</restUrl>
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
